use crate::buckets::models::PublicBucket;
use crate::buckets::private::PrivateBucket;
use crate::cache_system::models::CacheBucket;
use crate::handler::pipeline::api_engine::models::ApiEngineSettings;
use crate::handler::pipeline::bot_management::models::Bots;
use crate::handler::pipeline::caching::models::CacheLevel;
use crate::handler::pipeline::human_engine::models::HumanEngine;
use crate::handler::pipeline::rules::models::Rule;
use crate::models::analytics::Analytic;
use crate::models::analytics_by_example::AnalyticsByExampleDomain;
use crate::models::regions::Region;
use crate::utils::counter::Counter;
use dashmap::DashMap;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio_rustls::rustls::{Certificate, PrivateKey};
use url::Url;

#[derive(Debug)]
pub struct DomainContext {
    pub domain: String,
    pub origin: OriginSettings,
    pub api_engine_settings: ApiEngineSettings,
    // pub general_security_settings: SecuritySettings,
    pub caching_settings: CachingSettings,
    pub internal_settings: InternalSettings,
    pub bot_settings: BotManagementSettings,
    pub human_engine_settings: HumanEngine,
    pub buckets: Vec<Arc<PublicBucket>>, // this is where buckets are stored for this zone
    pub private_bucket: Arc<PrivateBucket>,
    pub analytic: Arc<Analytic>,
    pub rules: RulesSettings,
    // analytics by example
    pub analytics_by_example: AnalyticsByExampleDomain,
    // app settings - this is only set if the whole thing is an app
    pub app_settings: Option<Arc<AppSettings>>,
}

#[derive(Clone, Debug)]
pub struct RulesSettings {
    pub is_enabled: bool,
    pub rules: Vec<Rule>, // in the order they should be processed
}

#[derive(Clone, Debug)]
pub struct BotManagementSettings {
    pub enabled: bool,
    pub bots_allowed: Vec<Bots>,
}

#[derive(Clone, Debug)]
pub struct OriginSettings {
    pub settings: DashMap<String, Arc<OriginType>>, // map of the host and the appropriate hosts
    pub open_conns: Counter,                        // counter is used as a 'ratelimiter' here
}

#[derive(Clone, Debug)]
pub enum OriginType {
    Origin(Arc<OriginSetting>),
    App(String),
}

#[derive(Clone, Debug)]
pub struct AppSettings {
    // app settings

    // vector of regions. first is region the app is in, second is the counter
    // third is the machine ip, fourth is the machine host
    pub origins: Vec<(Region, (Counter, (Url, String)))>,

    // then, we have origin settings with an empty vector
    pub origin_settings: Arc<OriginSetting>,
}

#[derive(Clone, Debug)]
pub struct OriginSetting {
    pub origins: Vec<(Url, ((u8, Counter), Option<String>))>, // vector of origins and their respective weights along with pretend host if any should be used (this is how subdomains are connected)
    // TODO: let people choose origins by region
    // pub base_url: String, // we need this for websocket connections, because we are effectively 'combining' strings
    pub ssl: bool,
    pub http2: bool, // should we attempt to negotiate over http2? can use more resources, but could be faster.
    pub hits: Counter,
    pub errored: Counter,   // TODO implement
    pub is_localhost: bool, // if there's a localhost-like entry
    pub timeout: Duration,  // timeout before the request goes stale
    pub ip_data: bool, // would you like us to send IP data to your backend? should be a premium feature.
    pub keys: Option<Keys>,
}

#[derive(Clone, Debug)] // when doing keys, remove default
pub struct Keys {
    pub public_key: Vec<Certificate>,
    pub private_key: PrivateKey,
}
/*
pub struct SecuritySettings {
    pub request_inspection: bool,
    pub verified_bot_management: bool
}
 */

#[derive(Clone, Debug)]
pub struct CachingSettings {
    pub enabled: bool,
    pub default_cache_level: CacheLevel,
    pub default_cache_ttl: Duration,
    pub bucket: CacheBucket,
}

#[derive(Clone, Debug)]
pub struct InternalSettings {
    // can_cache is true if we can cache for this domain - used for internal ratelimiting
    pub can_cache: bool,
    // domain_blocked is true if the domain has been suspended
    pub domain_blocked: bool,
    // timeout for requests
    pub req_timeout: Duration,
    // threat score threshold
    pub threat_score_threshold: i16,
    // expected 'passed' requests
    pub expected_passed: i32,
    // expected query string requests
    pub expected_query_string: i32,
    // how big a cached file can be, in bytes / 1000
    pub cache_file_max: i32,
    // how big the total cache storage can be, in bytes / 1000
    pub total_cache_limit: i32,
    // allowed open connections to users backend, includes websocket requests
    pub allowed_open_conns: i32,
    // how many messages the server can send to the client in a 10 second period
    pub server_messages_threshold: i32,
    // how many requests all clients can send in 10 seconds time
    pub allowed_websocket_messages: i32,
    // bots allowed to this zone, totally
    pub bots_allowed: i32,
    // doesn't need to be implemented yet, the total request timeout (websockets). going to set at 24 hours
    pub total_request_timeout: Duration,
    // expected origin error requests
    pub expected_origin_errs: i32,
    // the amount of cache that can be attempted
    pub attempted_cache: i32,
}

// SystemSettings stores overarching system settings.
// This should be stored in an Arc
pub struct SystemSettings {
    // 2^32 - 1 threshold
    pub ban_threshold: AtomicU32,
    // quicker to ban IPs, requires all clients complete basic cookie challenge
    pub lockdown_enabled: AtomicBool,
    // multiplier for TCB (Total Cookie Brainfuck)
    pub multiplier: u32,
}
