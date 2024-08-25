use crate::buckets::models::PublicBucket;
use crate::buckets::private::PrivateBucket;
use crate::handler::pipeline::api_engine::models::{
    Action, ApiEngineSettings, Method, Rule, Setting, TriggerType, WsMethods,
};
use crate::handler::pipeline::bot_management::models::Bots;
use crate::handler::pipeline::human_engine::models::{
    HumanEngine, HumanEngineMode, InternalCounters,
};
use crate::handler::pipeline::human_engine::scores::failsafe::models::Failsafe;
use crate::handler::pipeline::rules::models::Trustbusting::{RatelimitBucket, SmartChallenge};
use crate::handler::pipeline::rules::models::{Match, Trigger};
use crate::models::analytics::Analytic;
use crate::models::analytics_by_example::AnalyticsByExampleDomain;
use crate::models::domain_context::{
    BotManagementSettings, CachingSettings, DomainContext, InternalSettings, OriginSetting,
    OriginSettings, OriginType,
};
use crate::utils::counter::Counter;
use crate::{CERTS, DOMAINS_DB};
use dashmap::DashMap;
use rustls_pemfile::{read_one, Item};
use std::collections::HashMap;
use std::iter;
use std::ops::Index;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio_rustls::rustls;
use tokio_rustls::rustls::sign::{CertifiedKey, RsaSigningKey};

pub fn debug_setup() {
    // For HTTPs, use Tokio - Rustls flavor
    let mut public_key_reader =
        std::io::BufReader::new(std::fs::File::open("/var/bigballs/opnieuw/api.pem").unwrap());
    let mut private_key_reader =
        std::io::BufReader::new(std::fs::File::open("/var/bigballs/opnieuw/apikey.pem").unwrap());

    let mut public_key: Vec<rustls::Certificate> = Vec::new(); // A vector for the fullchains of the individual certs
    let mut private_key: rustls::PrivateKey = rustls::PrivateKey(Vec::new());

    for item in iter::from_fn(|| read_one(&mut public_key_reader).transpose()) {
        match item.unwrap() {
            Item::X509Certificate(cert) => {
                public_key.push(rustls::Certificate(cert));
            }
            _ => panic!("wtf"),
        }
    }

    for item in iter::from_fn(|| read_one(&mut private_key_reader).transpose()) {
        // item is an enum
        match item.unwrap() {
            Item::PKCS8Key(cert) => {
                private_key = rustls::PrivateKey(cert);
            }
            _ => panic!("unhandled item"),
        }
    }

    CERTS.insert(
        "rizzle.chat".to_string(),
        Arc::new(CertifiedKey::new(
            public_key,
            std::sync::Arc::new(RsaSigningKey::new(&private_key).unwrap()),
        )),
    );

    let new_origin = DashMap::new();

    new_origin.insert(
        "rizzle.chat".to_string(),
        Arc::new(OriginType::Origin(Arc::new(OriginSetting {
            origins: Vec::from([(
                url::Url::from_str("http://193.176.244.250").unwrap(),
                (
                    (0, Counter::new()),
                    Some("packetware.apps.onpacketware.net".to_string()),
                ),
            )]),
            ssl: true,
            http2: false,
            hits: Counter::new(),
            errored: Counter::new(),
            is_localhost: false,
            timeout: Duration::from_secs(10),
            ip_data: false,
            keys: None,
        }))),
    );

    new_origin.insert(
        "api.rizzle.chat".to_string(),
        Arc::new(OriginType::Origin(Arc::new(OriginSetting {
            origins: Vec::from([(
                url::Url::from_str("https://rizzle-chat.edwardwc.workers.dev").unwrap(),
                (
                    (0, Counter::new()),
                    Some("rizzle-chat.edwardwc.workers.dev".to_string()),
                ),
            )]),
            ssl: false,
            http2: false,
            hits: Counter::new(),
            errored: Counter::new(),
            is_localhost: false,
            timeout: Duration::from_secs(10),
            ip_data: false,
            keys: None,
        }))),
    );

    let internal_settings = InternalSettings {
        can_cache: true,
        domain_blocked: false,
        req_timeout: Duration::new(10, 0), // requests to their backend
        threat_score_threshold: 100,
        expected_passed: 1000,
        expected_query_string: 1000,
        cache_file_max: 1240328060,
        total_cache_limit: 1240328070,
        allowed_open_conns: 4096,
        server_messages_threshold: 10,
        allowed_websocket_messages: 10,
        bots_allowed: 1000,
        total_request_timeout: Duration::from_secs(86400),
        expected_origin_errs: 1000,
        attempted_cache: 1000,
    };

    let mut buckets = Vec::new();

    buckets.push(PublicBucket::new(
        "Balls, biggest".to_string(),
        "69".to_string(),
        1000,
        2,
        false,
    ));

    DOMAINS_DB.insert(
        "rizzle.chat".to_string(),
        Arc::new(DomainContext {
            domain: "rizzle.chat".to_string(),
            origin: OriginSettings {
                settings: new_origin,
                open_conns: Default::default(),
            },
            api_engine_settings: ApiEngineSettings {
                enabled: true,
                strict_mode: true,
                rules: Vec::from([Setting {
                    host_path: "api.rizzle.chat".to_string(), // TODO make this include host
                    whitelist_factors: vec![],
                    rules: Vec::from([Rule {
                        id: "".to_string(),
                        position: 0,
                        hit: Counter::new(),
                        path: vec!["/ws/".to_string()],
                        trigger_type: TriggerType::StartsWith,
                        allow_query_string: false,
                        methods_allowed: Vec::from([Method::Ws(Vec::from([
                            WsMethods::Txt,
                            WsMethods::Ping,
                            WsMethods::Close,
                        ]))]),
                        action: Vec::from([
                            Action::Ratelimit(buckets.index(0).clone()), // TODO points-based ratelimiting
                        ]),
                        human_engine_validation: false,
                    }]),
                    open_api: true,
                }]),
            },
            caching_settings: CachingSettings {
                enabled: true,
                default_cache_level:
                    crate::handler::pipeline::caching::models::CacheLevel::Aggressive,
                default_cache_ttl: Duration::from_secs(1000),
                bucket: crate::cache_system::models::CacheBucket {
                    map: DashMap::new(),
                    allocations: Arc::new(RwLock::new(HashMap::new())),
                    total_size: Counter::new(),
                },
            },
            internal_settings: internal_settings.clone(),
            bot_settings: BotManagementSettings {
                enabled: true,
                bots_allowed: Vec::from([Bots::Googlebot]),
            },
            human_engine_settings: HumanEngine {
                mode: HumanEngineMode::Chill,
                internal_bucket: InternalCounters {
                    query_string_counter: Default::default(),
                    paths: Default::default(),
                },
                turbo_mode_enabled: true,
                failsafe: Arc::new(Failsafe {
                    current_count: Default::default(),
                    challenged: Default::default(),
                    rolling_average: 0,
                    enabled: Default::default(),
                    map: Default::default(),
                }),
            },
            buckets: buckets.clone(),
            private_bucket: Arc::new(PrivateBucket::new().setup_defaults(&internal_settings)),
            analytic: Arc::new(Analytic::new()),
            rules: crate::models::domain_context::RulesSettings {
                is_enabled: true,
                rules: Vec::from([crate::handler::pipeline::rules::models::Rule {
                    id: "696969".to_string(),
                    trigger: Trigger {
                        match_type: Vec::from([Match {
                            trigger: crate::handler::pipeline::rules::models::TriggerType::Any,
                            m_type: crate::handler::pipeline::rules::models::MatchType::UseStar,
                            inversed: false,
                            required: false,
                        }]),
                        trigger_requirement:
                            crate::handler::pipeline::rules::models::TriggerRequirement::One,
                        inversed: false,
                    },
                    action: crate::handler::pipeline::rules::models::Action::Trustbusting(
                        Vec::from([RatelimitBucket(buckets[0].clone()), SmartChallenge]),
                    ),
                    analytic: Default::default(),
                    enabled: true,
                    max: None,
                }]),
            },
            analytics_by_example: AnalyticsByExampleDomain {
                started: Counter::new(),
                examples: DashMap::new(),
            },
            app_settings: None,
        }),
    );
}
