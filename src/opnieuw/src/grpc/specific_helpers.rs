use std::{
    collections::HashMap,
    error::Error,
    io::{
        BufReader,
        Cursor,
    },
    iter,
    ops::DerefMut,
};

use crate::{
    buckets::models::PublicBucket,
    cache_system::models::CacheBucket,
    debug,
    grpc::all::{
        ApiEngineSettings as RpcApiEngineSettings,
        AppOriginSetting as RpcAppOriginSetting,
        AppOriginSetting,
        BotSettings as RpcBotSettings,
        BucketSettings as RpcBucketSettings,
        CachingSettings as RpcCachingSettings,
        Cert as RpcCert,
        HumanEngine as RpcHumanEngine,
        InternalSettings as RpcInternalSettings,
        OriginSetting as RpcOriginSetting,
        PageRules as RpcPageRules,
    },
    handler::pipeline::{
        api_engine::models::{
            Action,
            ApiEngineSettings,
            Method,
            Rule,
            Setting,
            TriggerType,
            WebMethods,
            WhitelistFactors,
            WsMethods,
        },
        bot_management::models::Bots,
        caching::models::CacheLevel,
        human_engine::{
            models::{
                HumanEngine,
                HumanEngineMode,
                InternalCounters,
            },
            scores::failsafe::models::Failsafe,
        },
        rules,
        rules::models::{
            Match,
            MatchType,
            Monopoly,
            Rule as PageRule,
            Trigger,
            TriggerRequirement,
            TriggerType as RuleTriggerType,
            Trustbusting,
        },
    },
    models::{
        domain_context::{
            AppSettings,
            BotManagementSettings,
            CachingSettings,
            InternalSettings,
            OriginSetting,
            OriginSettings,
            OriginType,
            RulesSettings,
        },
        regions::Region,
    },
    utils::counter::Counter,
    CERTS,
    WILDCARD_CERT,
};
use dashmap::DashMap;
use hyper::http::Method as HyperMethod;
use rustls_pemfile::{
    read_one,
    Item,
};
use std::{
    str::FromStr,
    sync::Arc,
    time::Duration,
};
use tokio::sync::RwLock;
use tokio_rustls::{
    rustls,
    rustls::sign::{
        CertifiedKey,
        RsaSigningKey,
    },
};
use url::Url;

pub struct RpcToRust {}

impl RpcToRust {
    pub fn origin_settings(
        os: HashMap<String, RpcOriginSetting>,
        old_origin_settings: Option<OriginSettings>,
    ) -> Result<OriginSettings, String> {
        let settings = DashMap::new();

        for i in os.iter() {
            if !i.1.app {
                let val = i.1;
                let mut origins = Vec::new();
                let _localhost = false; // TODO deprecate

                for v in i.1.origins.iter() {
                    if let Ok(url) = Url::from_str(&v.url) {
                        debug!("url: {:?}", url);
                        origins.push((
                            url,
                            ((v.weight as u8, Counter::new()), v.pretend_host.clone()),
                        ));
                    } else {
                        return Err("Couldn't parse origin to URL-format".into());
                    }
                }

                settings.insert(
                    i.0.clone(),
                    Arc::new(OriginType::Origin(Arc::new(OriginSetting {
                        origins,
                        ssl: val.ssl,
                        http2: val.http2,
                        hits: Counter::new(),
                        errored: Default::default(),
                        is_localhost: false,
                        timeout: Duration::from_secs(val.timeout as u64),
                        ip_data: val.ip_data,
                        keys: None, // TODO
                    }))),
                );
            } else {
                // now this is an app
                settings.insert(i.0.clone(), Arc::new(OriginType::App(i.1.app_name.clone())));
            }
        }

        match old_origin_settings {
            Some(t) => Ok(OriginSettings {
                settings,
                open_conns: Counter::from(t.open_conns.get()),
            }),
            None => Ok(OriginSettings {
                settings,
                open_conns: Counter::new(),
            }),
        }
    }

    pub fn app_settings(
        app_settings: Option<RpcAppOriginSetting>,
        _old_app_settings: Option<AppSettings>,
    ) -> Result<Option<Arc<AppSettings>>, String> {
        match app_settings {
            Some(t) => {
                let mut origins = Vec::new();

                for i in t.app_origin.iter() {
                    if let Ok(url) = Url::from_str(&i.ip) {
                        let region = match i.region {
                            0 => Region::ASH,
                            1 => Region::LAX,
                            2 => Region::AMS,
                            3 => Region::DAL,
                            _ => return Err("Region not implemented".into()),
                        };

                        origins.push((region, (Counter::new(), (url, i.host.clone()))));
                    } else {
                        return Err("Couldn't parse origin to URL-format".into());
                    }
                }

                let os = match t.origin_setting {
                    Some(os) => os,
                    None => return Err("No origin settings found".into()),
                };

                Ok(Some(Arc::new(AppSettings {
                    origins,
                    origin_settings: Arc::new(OriginSetting {
                        origins: vec![],
                        ssl: os.ssl,
                        http2: os.http2,
                        hits: Counter::new(),
                        errored: Counter::new(),
                        is_localhost: false,
                        timeout: Duration::from_secs(os.timeout as u64),
                        ip_data: false,
                        keys: None,
                    }),
                })))
            }
            None => Ok(None),
        }
    }

    pub fn api_engine(
        ae: RpcApiEngineSettings,
        _old_ae: Option<ApiEngineSettings>,
    ) -> Result<ApiEngineSettings, String> {
        let mut settings = Vec::new();

        for (k, v) in ae.settings.iter() {
            // get each setting
            let mut whitelist_factors = Vec::new();
            for (l, m) in v.kv_whitelist_factors.iter() {
                whitelist_factors.push(WhitelistFactors::Header(l.clone(), m.clone()))
            }
            for ip in v.ips.iter() {
                whitelist_factors.push(WhitelistFactors::Ip(ip.clone()))
            }

            let mut rules = Vec::new();

            for r in v.rules.iter() {
                let mut methods = Vec::new();

                let mut web_methods_allowed = Vec::new();

                for i in r.web_methods.iter() {
                    match i {
                        0 => web_methods_allowed.push(WebMethods::Get),
                        1 => web_methods_allowed.push(WebMethods::Post),
                        2 => web_methods_allowed.push(WebMethods::Options),
                        3 => web_methods_allowed.push(WebMethods::Delete),
                        4 => web_methods_allowed.push(WebMethods::Head),
                        5 => web_methods_allowed.push(WebMethods::Put),
                        6 => web_methods_allowed.push(WebMethods::Connect),
                        7 => web_methods_allowed.push(WebMethods::Trace),
                        8 => web_methods_allowed.push(WebMethods::Patch),
                        _ => return Err("gRPC web method unimplemented".into()),
                    }
                }

                if !web_methods_allowed.is_empty() {
                    methods.push(Method::Web(web_methods_allowed))
                }

                let mut ws_methods_allowed = Vec::new();

                for i in r.ws_methods.iter() {
                    ws_methods_allowed.push(match i {
                        0 => WsMethods::Txt,
                        1 => WsMethods::Ping,
                        2 => WsMethods::Binary,
                        3 => WsMethods::Close,
                        _ => return Err("gRPC web method unimplemented".into()),
                    })
                }

                if !ws_methods_allowed.is_empty() {
                    methods.push(Method::Ws(ws_methods_allowed))
                }

                let mut actions = Vec::new();

                if let Some(t) = r.cache_level {
                    let ttl = r.cache_level_ttl.map(|_m| Duration::from_secs(t as u64));

                    actions.push(Action::Cache(
                        match t {
                            0 => CacheLevel::None,
                            1 => CacheLevel::Standard,
                            2 => CacheLevel::IgnoreQueryString,
                            3 => CacheLevel::Aggressive,
                            _ => return Err("gRPC web method unimplemented".into()),
                        },
                        ttl,
                    ))
                }

                let trigger_type = match r.match_type {
                    0 => TriggerType::UseStar,
                    1 => TriggerType::Exact,
                    2 => TriggerType::Contains,
                    3 => TriggerType::StartsWith,
                    _ => return Err("gRPC web method unimplemented".into()),
                };

                rules.push(Rule {
                    id: r.id.clone(),
                    position: r.position as u16,
                    hit: Counter::new(),
                    path: r.path.clone(),
                    trigger_type,
                    allow_query_string: r.allow_query_string,
                    methods_allowed: methods,
                    action: actions,
                    human_engine_validation: r.human_engine_validation,
                });
            }

            settings.push(Setting {
                host_path: k.clone(),
                rules,
                whitelist_factors,
                open_api: v.open_api,
            });
        }

        Ok(ApiEngineSettings {
            enabled: ae.enabled,
            strict_mode: ae.strict_mode,
            rules: settings, // these are the "settings"
        })
    }

    pub fn human_engine(
        he: RpcHumanEngine,
        old_he: Option<HumanEngine>,
    ) -> Result<HumanEngine, String> {
        let mode = match he.mode {
            0 => HumanEngineMode::Chill,
            1 => HumanEngineMode::Standard,
            2 => HumanEngineMode::LudicrousBotMitigation, // TODO grpc implementation
            _ => return Err("gRPC mode unimplemented".into()),
        };

        match old_he {
            Some(t) => Ok(HumanEngine {
                mode,
                internal_bucket: t.internal_bucket,
                turbo_mode_enabled: he.turbo_mode_enabled,
                failsafe: t.failsafe,
            }),
            None => Ok(HumanEngine {
                mode,
                internal_bucket: InternalCounters {
                    query_string_counter: Default::default(),
                    paths: DashMap::new(),
                },
                turbo_mode_enabled: he.turbo_mode_enabled,
                failsafe: Arc::new(Failsafe {
                    current_count: Default::default(),
                    challenged: Default::default(),
                    rolling_average: 0,
                    enabled: Default::default(),
                    map: Default::default(),
                }),
            }),
        }
    }

    pub fn bots(b: RpcBotSettings) -> Result<BotManagementSettings, String> {
        let mut bots_allowed = Vec::new();

        for i in b.bots.iter() {
            bots_allowed.push(match i {
                0 => Bots::Googlebot,
                1 => Bots::Bingbot,
                2 => Bots::UptimeRobot,
                _ => return Err("Bot allowed unimplemented".to_string()),
            })
        }

        Ok(BotManagementSettings {
            enabled: b.enabled,
            bots_allowed,
        })
    }

    pub fn cache_settings(
        c: RpcCachingSettings,
        old_c: Option<CachingSettings>,
    ) -> Result<CachingSettings, String> {
        let default_cache_level = match c.cache_level {
            0 => CacheLevel::None,
            1 => CacheLevel::Standard,
            2 => CacheLevel::IgnoreQueryString,
            3 => CacheLevel::Aggressive,
            _ => return Err("grpc unimplemented cache level @ cache settings".to_string()),
        };

        match old_c {
            Some(t) => Ok(CachingSettings {
                enabled: c.enabled,
                default_cache_level,
                default_cache_ttl: Duration::from_secs(c.default_ttl as u64),
                bucket: CacheBucket {
                    map: t.bucket.map,
                    allocations: t.bucket.allocations,
                    total_size: t.bucket.total_size,
                },
            }),
            None => Ok(CachingSettings {
                enabled: c.enabled,
                default_cache_level,
                default_cache_ttl: Duration::from_secs(c.default_ttl as u64),
                bucket: CacheBucket {
                    map: DashMap::new(),
                    allocations: Arc::new(RwLock::new(HashMap::new())),
                    total_size: Counter::new(),
                },
            }),
        }
    }

    pub fn page_rules(
        pr: RpcPageRules,
        _old_c: Option<RulesSettings>,
        buckets: Vec<Arc<PublicBucket>>,
        origin_settings: OriginSettings,
    ) -> Result<RulesSettings, String> {
        let mut rules = Vec::new();

        for i in pr.rules.iter() {
            let mut match_type = Vec::new();

            for m in i.matches.iter() {
                let trigger = match m.trigger_type {
                    0 => {
                        if let Some(t) = m.pure_string.clone() {
                            RuleTriggerType::Ip(t)
                        } else {
                            return Err(
                                "Ip was indicated as a trigger, but no pure string was attached"
                                    .to_string(),
                            );
                        }
                    }
                    1 => {
                        if let Some(t) = m.pure_string.clone() {
                            RuleTriggerType::Path(t)
                        } else {
                            return Err(
                                "Path was indicated as a trigger, but no pure string was attached"
                                    .to_string(),
                            );
                        }
                    }
                    2 => {
                        if m.key_value.len() == 1 {
                            let mut value = None;
                            for (k, v) in m.key_value.iter() {
                                value = Some(RuleTriggerType::Query {
                                    key: k.clone(),
                                    value: v.clone(),
                                });
                            }
                            value.unwrap()
                        } else {
                            return Err("Query was indicated as a trigger, but there wasn't one entry supplied".to_string());
                        }
                    }
                    3 => {
                        if let Some(t) = m.pure_string.clone() {
                            RuleTriggerType::Asn(t)
                        } else {
                            return Err(
                                "ASN was indicated as a trigger, but no pure string was attached"
                                    .to_string(),
                            );
                        }
                    }
                    4 => {
                        if let Some(t) = m.pure_string.clone() {
                            RuleTriggerType::Country(t)
                        } else {
                            return Err("Country was indicated as a trigger, but no pure string was attached".to_string());
                        }
                    }
                    5 => {
                        if let Some(t) = m.pure_string.clone() {
                            RuleTriggerType::Continent(t)
                        } else {
                            return Err("Continent was indicated as a trigger, but no pure string was attached".to_string());
                        }
                    }
                    6 => {
                        if m.key_value.len() == 1 {
                            let mut value = None;
                            for (k, v) in m.key_value.iter() {
                                value = Some(RuleTriggerType::Headers {
                                    key: k.clone(),
                                    value: v.clone(),
                                });
                            }
                            value.unwrap()
                        } else {
                            return Err("Headers was indicated as a trigger, but there wasn't one entry supplied".to_string());
                        }
                    }
                    7 => {
                        if let Some(t) = m.pure_string.clone() {
                            RuleTriggerType::Host(t)
                        } else {
                            return Err(
                                "Host was indicated as a trigger, but no pure string was attached"
                                    .to_string(),
                            );
                        }
                    }
                    8 => {
                        if let Some(t) = m.pure_string.clone() {
                            let method = match t.as_str() {
                                "GET" => HyperMethod::GET,
                                "POST" => HyperMethod::POST,
                                "PUT" => HyperMethod::PUT,
                                "DELETE" => HyperMethod::DELETE,
                                "PATCH" => HyperMethod::PATCH,
                                "OPTIONS" => HyperMethod::OPTIONS,
                                "HEAD" => HyperMethod::HEAD,
                                _ => {
                                    return Err(
                                        "Unexpected HTTP method selected as trigger".to_string()
                                    )
                                }
                            };

                            RuleTriggerType::Method(method)
                        } else {
                            return Err(
                                "Host was indicated as a trigger, but no pure string was attached"
                                    .to_string(),
                            );
                        }
                    }
                    9 => {
                        if let Some(t) = m.pure_string.clone() {
                            RuleTriggerType::UserAgent(t)
                        } else {
                            return Err("User agent was indicated as a trigger, but no pure string was attached".to_string());
                        }
                    }
                    10 => {
                        if let Some(t) = m.pure_string.clone() {
                            RuleTriggerType::Cookie(t)
                        } else {
                            return Err("Cookie was indicated as a trigger, but no pure string was attached".to_string());
                        }
                    }
                    11 => RuleTriggerType::Any,
                    _ => return Err("Rule trigger type is unimplemented".to_string()),
                };

                let m_type = match m.match_type {
                    0 => MatchType::UseStar,
                    1 => MatchType::Exact,
                    2 => MatchType::Contains,
                    3 => MatchType::StartsWith,
                    _ => return Err("Match type unimplemented".to_string()),
                };

                match_type.push(Match {
                    trigger,
                    m_type,
                    inversed: m.inversed,
                    required: m.required,
                })
            }

            let trigger_requirement = match i.trigger_requirement.to_lowercase().as_str() {
                "one" => TriggerRequirement::One,
                "all" => TriggerRequirement::All,
                "multiple" => TriggerRequirement::Multiple(i.trigger_amount as u32),
                _ => return Err("Page rule trigger requirement unimplemented".to_string()),
            };

            let action = match i.action {
                0 => {
                    // monopoly action
                    let monopoly = match i.monopoly_action {
                        Some(0) => Monopoly::Block,
                        _ => {
                            return Err(format!(
                                "grpc monopoly action unimplemented: {:?}",
                                i.monopoly_action
                            ))
                        }
                    };

                    rules::models::Action::Monopoly(monopoly)
                }
                1 => {
                    // trustbust action
                    let mut trustbusting = Vec::new();

                    for k in i.trustbust_option.iter() {
                        trustbusting.push(match k {
                            0 => Trustbusting::SmartChallenge,
                            1 => Trustbusting::CaptchaChallenge,
                            2 => {
                                debug!("bucket selected");

                                let mut bucket = None;

                                for v in buckets.iter() {
                                    debug!("public bucket: {}, {}", v.bucket.name, i.bucket_name.clone().unwrap());
                                    if v.bucket.name == i.bucket_name.clone().unwrap() {
                                        bucket = Some(v.clone())
                                    }
                                }

                                if bucket.is_none() {
                                    debug!("bucket was none");

                                    return Err("couldn't find that bucket".to_string())
                                }

                                Trustbusting::RatelimitBucket(bucket.unwrap())
                            },
                            3 => {
                                if let Some(cl) = i.cache_level {
                                    let cache_level = match cl {
                                        0 => CacheLevel::None,
                                        1 => CacheLevel::Standard,
                                        2 => CacheLevel::IgnoreQueryString,
                                        3 => CacheLevel::Aggressive,
                                        _ => return Err("Cache level unimplemented".to_string())
                                    };

                                    let ttl = i.cache_ttl.map(|t| Duration::from_secs(t as u64));

                                    Trustbusting::Cache(cache_level, ttl)
                                } else {
                                    return Err("Cache level was selected as an option but no cache level was provided".to_string())
                                }
                            },
                            4 => {
                                // redirect
                                if let Some(t) = i.redirect.clone() {
                                    Trustbusting::Redirect(t)
                                } else {
                                    return Err("Redirection was selected, but no redirect URL was indicated".to_string())
                                }
                            },
                            5 => {
                                if let Some(t) = i.app_name.clone() {
                                    Trustbusting::UseApp(t)
                                } else {
                                    // use backend
                                    if let Some(t) = i.backend_host.clone() {
                                        if let Some(_m) = origin_settings.settings.get(&t) {
                                            Trustbusting::UseBackend(t)
                                        } else {
                                            return Err("Backend host was selected, but the origin setting couldn't be found".to_string())
                                        }
                                    } else {
                                        return Err("Using a specific backend was indicated, but no backend or app was provided".to_string())
                                    }
                                }
                            }
                            _ => return Err("Trustbust option didn't exist".to_string())
                        })
                    }

                    rules::models::Action::Trustbusting(trustbusting)
                }
                _ => return Err("grpc page rule action type unimplemented".to_string()),
            };

            let mut max = None;

            if i.rule_max > 0 {
                max = Some(i.rule_max as u32);
            }

            rules.push(PageRule {
                id: i.id.clone(),
                trigger: Trigger {
                    match_type,
                    trigger_requirement,
                    inversed: i.inversed,
                },
                action,
                analytic: Counter::new(),
                enabled: i.enabled,
                max,
            });
        }

        Ok(RulesSettings {
            is_enabled: pr.enabled,
            rules,
        })
    }

    pub fn internal_settings(is: RpcInternalSettings) -> Result<InternalSettings, String> {
        Ok(InternalSettings {
            can_cache: is.can_cache,
            domain_blocked: is.domain_blocked,
            req_timeout: Duration::from_secs(is.request_timeout as u64),
            threat_score_threshold: is.threat_score_threshold as i16,
            expected_passed: is.expected_passed,
            expected_query_string: 0, // TODO
            cache_file_max: is.cache_file_max,
            total_cache_limit: is.total_cache_limit,
            allowed_open_conns: is.allowed_open_conns,
            server_messages_threshold: is.server_messages_threshold,
            allowed_websocket_messages: is.allowed_ws_messages,
            bots_allowed: 100,
            total_request_timeout: Duration::from_secs(86400),
            expected_origin_errs: is.expected_errored,
            attempted_cache: is.cache_attempted,
        })
    }

    pub fn bucket_settings(b: RpcBucketSettings) -> Result<Vec<Arc<PublicBucket>>, String> {
        let mut public_buckets = Vec::new();

        for i in b.buckets {
            let timeout = i.timeout as u32 / 10;

            public_buckets.push(PublicBucket::new(
                i.name,
                i.id,
                i.threshold as u32,
                timeout,
                i.premium,
            ));
        }

        Ok(public_buckets)
    }

    pub fn cert(b: RpcCert) -> Result<(), Box<dyn Error>> {
        let mut public_key_reader = BufReader::new(Cursor::new(b.full_chain));
        let mut private_key_reader = BufReader::new(Cursor::new(b.private_key));

        let mut public_key: Vec<rustls::Certificate> = Vec::new(); // A vector for the fullchains of the individual certs
        let mut private_key: rustls::PrivateKey = rustls::PrivateKey(Vec::new());

        for item in iter::from_fn(|| read_one(&mut public_key_reader).transpose()) {
            match item.unwrap() {
                Item::X509Certificate(cert) => {
                    public_key.push(rustls::Certificate(cert));
                }
                _ => return Err("unexpected fullchain type")?,
            }
        }

        for item in iter::from_fn(|| read_one(&mut private_key_reader).transpose()) {
            // item is an enum
            match item.unwrap() {
                Item::PKCS8Key(cert) => {
                    private_key = rustls::PrivateKey(cert);
                }
                _ => return Err("unexpected private key type")?,
            }
        }

        let signing_key = match RsaSigningKey::new(&private_key) {
            Ok(t) => t,
            Err(e) => return Err(format!("error parsing into private key: {}", e).as_str())?,
        };

        if !b.wildcard {
            CERTS.insert(
                b.domain.clone(),
                Arc::new(CertifiedKey::new(public_key, Arc::new(signing_key))),
            );
        } else {
            let mut write = WILDCARD_CERT.write().unwrap();

            *write = Some(Arc::new(CertifiedKey::new(
                public_key,
                Arc::new(signing_key),
            )));

            println!("Wildcard cert added");
        }

        Ok(())
    }
}
