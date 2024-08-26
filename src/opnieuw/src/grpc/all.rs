/// the new challenge
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartChallengeScript {
    #[prost(string, tag = "1")]
    pub smart_challenge_script: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub bg_challenge_script: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearCacheMessage {
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
    #[prost(bool, optional, tag = "2")]
    pub everything: ::core::option::Option<bool>,
    #[prost(string, repeated, tag = "3")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "CacheClearType", optional, tag = "4")]
    pub r#type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cert {
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub full_chain: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub private_key: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub wildcard: bool,
}
/// THE MAIN THINGS
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllDomainSchema {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(message, repeated, tag = "2")]
    pub everything: ::prost::alloc::vec::Vec<FullDomainSchema>,
    #[prost(message, repeated, tag = "3")]
    pub keys: ::prost::alloc::vec::Vec<Cert>,
    #[prost(message, optional, tag = "4")]
    pub challenge: ::core::option::Option<SmartChallengeScript>,
}
/// for when a domain is updated, only the difference is sent
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartialDomainSchema {
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
    /// if this isn't part of it, then it'll just be empty
    #[prost(map = "string, message", tag = "2")]
    pub origin_settings: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        OriginSetting,
    >,
    #[prost(message, optional, tag = "3")]
    pub api_engine_settings: ::core::option::Option<ApiEngineSettings>,
    #[prost(message, optional, tag = "4")]
    pub human_engine_settings: ::core::option::Option<HumanEngine>,
    #[prost(message, optional, tag = "5")]
    pub bot_settings: ::core::option::Option<BotSettings>,
    #[prost(message, optional, tag = "6")]
    pub cache_settings: ::core::option::Option<CachingSettings>,
    #[prost(message, optional, tag = "7")]
    pub buckets: ::core::option::Option<BucketSettings>,
    #[prost(message, optional, tag = "8")]
    pub page_rules: ::core::option::Option<PageRules>,
    #[prost(message, optional, tag = "9")]
    pub internal_settings: ::core::option::Option<InternalSettings>,
    #[prost(message, optional, tag = "10")]
    pub app_origin_settings: ::core::option::Option<AppOriginSetting>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullDomainSchema {
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
    #[prost(map = "string, message", tag = "2")]
    pub origin_settings: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        OriginSetting,
    >,
    #[prost(message, optional, tag = "3")]
    pub api_engine_settings: ::core::option::Option<ApiEngineSettings>,
    #[prost(message, optional, tag = "4")]
    pub human_engine_settings: ::core::option::Option<HumanEngine>,
    #[prost(message, optional, tag = "5")]
    pub bot_settings: ::core::option::Option<BotSettings>,
    #[prost(message, optional, tag = "6")]
    pub cache_settings: ::core::option::Option<CachingSettings>,
    #[prost(message, optional, tag = "7")]
    pub buckets: ::core::option::Option<BucketSettings>,
    #[prost(message, optional, tag = "8")]
    pub page_rules: ::core::option::Option<PageRules>,
    #[prost(message, optional, tag = "9")]
    pub internal_settings: ::core::option::Option<InternalSettings>,
    #[prost(message, optional, tag = "10")]
    pub app_origin_settings: ::core::option::Option<AppOriginSetting>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDomainSchema {
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
}
/// HUMAN ENGINE SETTINGS
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HumanEngine {
    #[prost(enumeration = "HumanEngineMode", tag = "1")]
    pub mode: i32,
    #[prost(bool, tag = "2")]
    pub turbo_mode_enabled: bool,
    /// notifications are handled by the API and thus don't need to be shared with the proxy servers
    #[prost(string, tag = "3")]
    pub traffic_level: ::prost::alloc::string::String,
}
/// ORIGINS
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OriginSetting {
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub ssl: bool,
    #[prost(bool, tag = "3")]
    pub http2: bool,
    #[prost(sint32, tag = "4")]
    pub timeout: i32,
    #[prost(bool, tag = "6")]
    pub ip_data: bool,
    #[prost(bool, tag = "7")]
    pub origin_failover: bool,
    /// keys
    #[prost(bytes = "vec", repeated, tag = "8")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", tag = "9")]
    pub private_key: ::prost::alloc::vec::Vec<u8>,
    /// origins themselves
    #[prost(message, repeated, tag = "10")]
    pub origins: ::prost::alloc::vec::Vec<Origin>,
    /// if it's an app or not
    #[prost(bool, tag = "11")]
    pub app: bool,
    /// if it's an app, it will be this one
    #[prost(string, tag = "12")]
    pub app_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Origin {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(sint32, tag = "2")]
    pub weight: i32,
    #[prost(string, optional, tag = "3")]
    pub pretend_host: ::core::option::Option<::prost::alloc::string::String>,
}
/// App origin settings
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppOriginSetting {
    #[prost(message, repeated, tag = "1")]
    pub app_origin: ::prost::alloc::vec::Vec<AppOrigin>,
    /// origin setting
    #[prost(message, optional, tag = "2")]
    pub origin_setting: ::core::option::Option<OriginSetting>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppOrigin {
    #[prost(enumeration = "Region", tag = "1")]
    pub region: i32,
    #[prost(string, tag = "2")]
    pub ip: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub host: ::prost::alloc::string::String,
}
/// API ENGINE
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiEngineSettings {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(bool, tag = "2")]
    pub strict_mode: bool,
    /// key is host + path
    #[prost(map = "string, message", tag = "3")]
    pub settings: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ApiEngineSetting,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiEngineSetting {
    /// key_value header whitelist factors for this path
    #[prost(map = "string, string", tag = "1")]
    pub kv_whitelist_factors: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, repeated, tag = "2")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "3")]
    pub open_api: bool,
    #[prost(message, repeated, tag = "4")]
    pub rules: ::prost::alloc::vec::Vec<ApiEngineRule>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiEngineRule {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(sint32, tag = "2")]
    pub position: i32,
    #[prost(string, repeated, tag = "3")]
    pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// either "Exact", "Contains", or "StartsWith"
    #[prost(enumeration = "MatchType", tag = "4")]
    pub match_type: i32,
    #[prost(bool, tag = "5")]
    pub allow_query_string: bool,
    #[prost(enumeration = "WsMethods", repeated, tag = "6")]
    pub ws_methods: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration = "WebMethods", repeated, tag = "7")]
    pub web_methods: ::prost::alloc::vec::Vec<i32>,
    /// {domain}_{id} format
    #[prost(string, optional, tag = "8")]
    pub ratelimiting_bucket: ::core::option::Option<::prost::alloc::string::String>,
    /// None, Standard, IgnoreQueryString or Aggressive
    #[prost(enumeration = "CacheLevel", optional, tag = "9")]
    pub cache_level: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag = "10")]
    pub cache_level_ttl: ::core::option::Option<i32>,
    /// human engine validation
    #[prost(bool, tag = "11")]
    pub human_engine_validation: bool,
}
/// CACHING SETTINGS
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CachingSettings {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(enumeration = "CacheLevel", tag = "2")]
    pub cache_level: i32,
    #[prost(sint32, tag = "3")]
    pub default_ttl: i32,
}
/// BOT SETTINGS
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BotSettings {
    /// if the management is enabled
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// the allowed bots
    #[prost(enumeration = "Bot", repeated, tag = "2")]
    pub bots: ::prost::alloc::vec::Vec<i32>,
}
/// RATELIMITING BUCKET SETTINGS
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketSettings {
    #[prost(message, repeated, tag = "1")]
    pub buckets: ::prost::alloc::vec::Vec<Bucket>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bucket {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub premium: bool,
    #[prost(sint32, tag = "4")]
    pub threshold: i32,
    #[prost(sint32, tag = "5")]
    pub timeout: i32,
}
/// PAGE RULES
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageRules {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(message, repeated, tag = "2")]
    pub rules: ::prost::alloc::vec::Vec<PageRule>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageRule {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(sint32, tag = "2")]
    pub order: i32,
    /// things it can be matched on!
    #[prost(message, repeated, tag = "3")]
    pub matches: ::prost::alloc::vec::Vec<Match>,
    /// either one, all, or multiple
    #[prost(string, tag = "4")]
    pub trigger_requirement: ::prost::alloc::string::String,
    /// if multiple is selected -- the amount of matches required to trigger the action
    #[prost(sint32, tag = "5")]
    pub trigger_amount: i32,
    #[prost(bool, tag = "6")]
    pub inversed: bool,
    #[prost(enumeration = "Action", tag = "7")]
    pub action: i32,
    /// if action is monopoly ...
    #[prost(enumeration = "MonopolyAction", optional, tag = "8")]
    pub monopoly_action: ::core::option::Option<i32>,
    /// if action is a trustbust ...
    #[prost(enumeration = "TrustBustOption", repeated, tag = "9")]
    pub trustbust_option: ::prost::alloc::vec::Vec<i32>,
    /// special action information
    /// buckets
    #[prost(string, optional, tag = "10")]
    pub bucket_name: ::core::option::Option<::prost::alloc::string::String>,
    /// caching
    #[prost(enumeration = "CacheLevel", optional, tag = "11")]
    pub cache_level: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag = "12")]
    pub cache_ttl: ::core::option::Option<i32>,
    /// redirect to
    #[prost(string, optional, tag = "13")]
    pub redirect: ::core::option::Option<::prost::alloc::string::String>,
    /// use backend, fetch from host
    #[prost(string, optional, tag = "14")]
    pub backend_host: ::core::option::Option<::prost::alloc::string::String>,
    /// if the backend host is an app
    #[prost(string, optional, tag = "15")]
    pub app_name: ::core::option::Option<::prost::alloc::string::String>,
    /// if the rule is enabled
    #[prost(bool, tag = "16")]
    pub enabled: bool,
    /// the rule max (if it's enabled, -1 if not)
    #[prost(sint32, tag = "17")]
    pub rule_max: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Match {
    /// ip, path, bunch of other cool stuff
    #[prost(enumeration = "TriggerType", tag = "1")]
    pub trigger_type: i32,
    #[prost(enumeration = "MatchType", tag = "2")]
    pub match_type: i32,
    /// the string we are going to compare the fetched information to
    #[prost(string, optional, tag = "3")]
    pub pure_string: ::core::option::Option<::prost::alloc::string::String>,
    /// or optionally, key value data. empty map if it doesn't use key value data.
    #[prost(map = "string, string", tag = "4")]
    pub key_value: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// special options
    #[prost(bool, tag = "5")]
    pub inversed: bool,
    #[prost(bool, tag = "6")]
    pub required: bool,
}
/// INTERNAL SETTINGS
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalSettings {
    #[prost(bool, tag = "2")]
    pub can_cache: bool,
    #[prost(bool, tag = "3")]
    pub domain_blocked: bool,
    /// uploads and such
    #[prost(sint32, tag = "4")]
    pub request_timeout: i32,
    #[prost(sint32, tag = "5")]
    pub threat_score_threshold: i32,
    /// expected figures
    #[prost(sint32, tag = "6")]
    pub expected_passed: i32,
    #[prost(sint32, tag = "7")]
    pub expected_errored: i32,
    /// bytes / 1000
    #[prost(sint32, tag = "8")]
    pub cache_file_max: i32,
    #[prost(sint32, tag = "9")]
    pub total_cache_limit: i32,
    #[prost(sint32, tag = "10")]
    pub allowed_open_conns: i32,
    /// ratelimit serv
    #[prost(sint32, tag = "11")]
    pub server_messages_threshold: i32,
    /// the amount of allowed websocket connections in 10 seconds
    #[prost(sint32, tag = "12")]
    pub allowed_ws_messages: i32,
    /// how much a domain can attempt to cache
    #[prost(sint32, tag = "13")]
    pub cache_attempted: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CacheClearType {
    ExactPath = 0,
    ContainsPath = 1,
    StartsWithPath = 2,
    EndsWithPath = 3,
}
impl CacheClearType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CacheClearType::ExactPath => "ExactPath",
            CacheClearType::ContainsPath => "ContainsPath",
            CacheClearType::StartsWithPath => "StartsWithPath",
            CacheClearType::EndsWithPath => "EndsWithPath",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ExactPath" => Some(Self::ExactPath),
            "ContainsPath" => Some(Self::ContainsPath),
            "StartsWithPath" => Some(Self::StartsWithPath),
            "EndsWithPath" => Some(Self::EndsWithPath),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HumanEngineMode {
    Chill = 0,
    StandardMode = 1,
    StandardPlus = 2,
}
impl HumanEngineMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HumanEngineMode::Chill => "Chill",
            HumanEngineMode::StandardMode => "StandardMode",
            HumanEngineMode::StandardPlus => "StandardPlus",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Chill" => Some(Self::Chill),
            "StandardMode" => Some(Self::StandardMode),
            "StandardPlus" => Some(Self::StandardPlus),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Region {
    Ash = 0,
    Lax = 1,
    Ams = 2,
    Dal = 3,
}
impl Region {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Region::Ash => "ASH",
            Region::Lax => "LAX",
            Region::Ams => "AMS",
            Region::Dal => "DAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASH" => Some(Self::Ash),
            "LAX" => Some(Self::Lax),
            "AMS" => Some(Self::Ams),
            "DAL" => Some(Self::Dal),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WsMethods {
    Txt = 0,
    Ping = 1,
    Binary = 2,
    Close = 3,
}
impl WsMethods {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WsMethods::Txt => "Txt",
            WsMethods::Ping => "Ping",
            WsMethods::Binary => "Binary",
            WsMethods::Close => "Close",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Txt" => Some(Self::Txt),
            "Ping" => Some(Self::Ping),
            "Binary" => Some(Self::Binary),
            "Close" => Some(Self::Close),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WebMethods {
    Get = 0,
    Post = 1,
    Options = 2,
    Delete = 3,
    Head = 4,
    Put = 5,
    Connect = 6,
    Trace = 7,
    Patch = 8,
}
impl WebMethods {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WebMethods::Get => "Get",
            WebMethods::Post => "Post",
            WebMethods::Options => "Options",
            WebMethods::Delete => "Delete",
            WebMethods::Head => "Head",
            WebMethods::Put => "Put",
            WebMethods::Connect => "Connect",
            WebMethods::Trace => "Trace",
            WebMethods::Patch => "Patch",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Get" => Some(Self::Get),
            "Post" => Some(Self::Post),
            "Options" => Some(Self::Options),
            "Delete" => Some(Self::Delete),
            "Head" => Some(Self::Head),
            "Put" => Some(Self::Put),
            "Connect" => Some(Self::Connect),
            "Trace" => Some(Self::Trace),
            "Patch" => Some(Self::Patch),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Actions {
    RatelimitApi = 0,
    CacheApi = 1,
}
impl Actions {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Actions::RatelimitApi => "RatelimitAPI",
            Actions::CacheApi => "CacheAPI",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RatelimitAPI" => Some(Self::RatelimitApi),
            "CacheAPI" => Some(Self::CacheApi),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CacheLevel {
    None = 0,
    Standard = 1,
    IgnoreQueryString = 2,
    Aggressive = 3,
}
impl CacheLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CacheLevel::None => "None",
            CacheLevel::Standard => "Standard",
            CacheLevel::IgnoreQueryString => "IgnoreQueryString",
            CacheLevel::Aggressive => "Aggressive",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "None" => Some(Self::None),
            "Standard" => Some(Self::Standard),
            "IgnoreQueryString" => Some(Self::IgnoreQueryString),
            "Aggressive" => Some(Self::Aggressive),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Bot {
    Googlebot = 0,
    Bingbot = 1,
    UptimeRobot = 2,
}
impl Bot {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Bot::Googlebot => "Googlebot",
            Bot::Bingbot => "Bingbot",
            Bot::UptimeRobot => "UptimeRobot",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Googlebot" => Some(Self::Googlebot),
            "Bingbot" => Some(Self::Bingbot),
            "UptimeRobot" => Some(Self::UptimeRobot),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TriggerType {
    Ip = 0,
    Path = 1,
    Query = 2,
    Asn = 3,
    Country = 4,
    Continent = 5,
    Headers = 6,
    Host = 7,
    Method = 8,
    UserAgent = 9,
    Cookie = 10,
    Any = 11,
}
impl TriggerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TriggerType::Ip => "Ip",
            TriggerType::Path => "Path",
            TriggerType::Query => "Query",
            TriggerType::Asn => "Asn",
            TriggerType::Country => "Country",
            TriggerType::Continent => "Continent",
            TriggerType::Headers => "Headers",
            TriggerType::Host => "Host",
            TriggerType::Method => "Method",
            TriggerType::UserAgent => "UserAgent",
            TriggerType::Cookie => "Cookie",
            TriggerType::Any => "Any",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Ip" => Some(Self::Ip),
            "Path" => Some(Self::Path),
            "Query" => Some(Self::Query),
            "Asn" => Some(Self::Asn),
            "Country" => Some(Self::Country),
            "Continent" => Some(Self::Continent),
            "Headers" => Some(Self::Headers),
            "Host" => Some(Self::Host),
            "Method" => Some(Self::Method),
            "UserAgent" => Some(Self::UserAgent),
            "Cookie" => Some(Self::Cookie),
            "Any" => Some(Self::Any),
            _ => None,
        }
    }
}
/// can be shared for API engine as well
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MatchType {
    UseStar = 0,
    Exact = 1,
    Contains = 2,
    StartsWith = 3,
}
impl MatchType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MatchType::UseStar => "UseStar",
            MatchType::Exact => "Exact",
            MatchType::Contains => "Contains",
            MatchType::StartsWith => "StartsWith",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UseStar" => Some(Self::UseStar),
            "Exact" => Some(Self::Exact),
            "Contains" => Some(Self::Contains),
            "StartsWith" => Some(Self::StartsWith),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    Monopoly = 0,
    Trustbusting = 1,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::Monopoly => "Monopoly",
            Action::Trustbusting => "Trustbusting",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Monopoly" => Some(Self::Monopoly),
            "Trustbusting" => Some(Self::Trustbusting),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MonopolyAction {
    Block = 0,
}
impl MonopolyAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MonopolyAction::Block => "Block",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Block" => Some(Self::Block),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrustBustOption {
    SmartChallenge = 0,
    CaptchaChallenge = 1,
    RatelimitBucket = 2,
    Cache = 3,
    Redirect = 4,
    UseBackend = 5,
    SkipHumanEngine = 6,
}
impl TrustBustOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrustBustOption::SmartChallenge => "SmartChallenge",
            TrustBustOption::CaptchaChallenge => "CaptchaChallenge",
            TrustBustOption::RatelimitBucket => "RatelimitBucket",
            TrustBustOption::Cache => "Cache",
            TrustBustOption::Redirect => "Redirect",
            TrustBustOption::UseBackend => "UseBackend",
            TrustBustOption::SkipHumanEngine => "SkipHumanEngine",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SmartChallenge" => Some(Self::SmartChallenge),
            "CaptchaChallenge" => Some(Self::CaptchaChallenge),
            "RatelimitBucket" => Some(Self::RatelimitBucket),
            "Cache" => Some(Self::Cache),
            "Redirect" => Some(Self::Redirect),
            "UseBackend" => Some(Self::UseBackend),
            "SkipHumanEngine" => Some(Self::SkipHumanEngine),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod big_baller_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BigBallerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BigBallerClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BigBallerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BigBallerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            BigBallerClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// on startup message, includes all domains
        pub async fn all_domains(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryResponse>,
        ) -> std::result::Result<
            tonic::Response<super::AllDomainSchema>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/all.BigBaller/AllDomains");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("all.BigBaller", "AllDomains"));
            self.inner.unary(req, path, codec).await
        }
        /// update domain after the user, or an admin, makes some changes
        pub async fn update_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::PartialDomainSchema>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/all.BigBaller/UpdateDomain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("all.BigBaller", "UpdateDomain"));
            self.inner.unary(req, path, codec).await
        }
        /// new domain on our service
        pub async fn new_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::FullDomainSchema>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/all.BigBaller/NewDomain");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("all.BigBaller", "NewDomain"));
            self.inner.unary(req, path, codec).await
        }
        /// deleting a domain from our service
        pub async fn delete_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDomainSchema>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/all.BigBaller/DeleteDomain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("all.BigBaller", "DeleteDomain"));
            self.inner.unary(req, path, codec).await
        }
        /// clear cache
        pub async fn clear_cache(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearCacheMessage>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/all.BigBaller/ClearCache");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("all.BigBaller", "ClearCache"));
            self.inner.unary(req, path, codec).await
        }
        /// SSL
        pub async fn challenge(
            &mut self,
            request: impl tonic::IntoRequest<super::Token>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/all.BigBaller/Challenge");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("all.BigBaller", "Challenge"));
            self.inner.unary(req, path, codec).await
        }
        /// Challenge removal
        pub async fn challenge_removal(
            &mut self,
            request: impl tonic::IntoRequest<super::Token>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/all.BigBaller/ChallengeRemoval",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("all.BigBaller", "ChallengeRemoval"));
            self.inner.unary(req, path, codec).await
        }
        /// From proxy > main server
        pub async fn challenge_completed(
            &mut self,
            request: impl tonic::IntoRequest<super::Cert>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/all.BigBaller/ChallengeCompleted",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("all.BigBaller", "ChallengeCompleted"));
            self.inner.unary(req, path, codec).await
        }
        /// Challenge shuffler
        pub async fn refresh_challenge(
            &mut self,
            request: impl tonic::IntoRequest<super::SmartChallengeScript>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/all.BigBaller/RefreshChallenge",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("all.BigBaller", "RefreshChallenge"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod big_baller_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BigBallerServer.
    #[async_trait]
    pub trait BigBaller: Send + Sync + 'static {
        /// on startup message, includes all domains
        async fn all_domains(
            &self,
            request: tonic::Request<super::QueryResponse>,
        ) -> std::result::Result<tonic::Response<super::AllDomainSchema>, tonic::Status>;
        /// update domain after the user, or an admin, makes some changes
        async fn update_domain(
            &self,
            request: tonic::Request<super::PartialDomainSchema>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status>;
        /// new domain on our service
        async fn new_domain(
            &self,
            request: tonic::Request<super::FullDomainSchema>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status>;
        /// deleting a domain from our service
        async fn delete_domain(
            &self,
            request: tonic::Request<super::DeleteDomainSchema>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status>;
        /// clear cache
        async fn clear_cache(
            &self,
            request: tonic::Request<super::ClearCacheMessage>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status>;
        /// SSL
        async fn challenge(
            &self,
            request: tonic::Request<super::Token>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status>;
        /// Challenge removal
        async fn challenge_removal(
            &self,
            request: tonic::Request<super::Token>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status>;
        /// From proxy > main server
        async fn challenge_completed(
            &self,
            request: tonic::Request<super::Cert>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status>;
        /// Challenge shuffler
        async fn refresh_challenge(
            &self,
            request: tonic::Request<super::SmartChallengeScript>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct BigBallerServer<T: BigBaller> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BigBaller> BigBallerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BigBallerServer<T>
    where
        T: BigBaller,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/all.BigBaller/AllDomains" => {
                    #[allow(non_camel_case_types)]
                    struct AllDomainsSvc<T: BigBaller>(pub Arc<T>);
                    impl<T: BigBaller> tonic::server::UnaryService<super::QueryResponse>
                    for AllDomainsSvc<T> {
                        type Response = super::AllDomainSchema;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryResponse>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).all_domains(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AllDomainsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/all.BigBaller/UpdateDomain" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDomainSvc<T: BigBaller>(pub Arc<T>);
                    impl<
                        T: BigBaller,
                    > tonic::server::UnaryService<super::PartialDomainSchema>
                    for UpdateDomainSvc<T> {
                        type Response = super::QueryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PartialDomainSchema>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_domain(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateDomainSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/all.BigBaller/NewDomain" => {
                    #[allow(non_camel_case_types)]
                    struct NewDomainSvc<T: BigBaller>(pub Arc<T>);
                    impl<
                        T: BigBaller,
                    > tonic::server::UnaryService<super::FullDomainSchema>
                    for NewDomainSvc<T> {
                        type Response = super::QueryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FullDomainSchema>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_domain(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewDomainSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/all.BigBaller/DeleteDomain" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDomainSvc<T: BigBaller>(pub Arc<T>);
                    impl<
                        T: BigBaller,
                    > tonic::server::UnaryService<super::DeleteDomainSchema>
                    for DeleteDomainSvc<T> {
                        type Response = super::QueryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDomainSchema>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_domain(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteDomainSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/all.BigBaller/ClearCache" => {
                    #[allow(non_camel_case_types)]
                    struct ClearCacheSvc<T: BigBaller>(pub Arc<T>);
                    impl<
                        T: BigBaller,
                    > tonic::server::UnaryService<super::ClearCacheMessage>
                    for ClearCacheSvc<T> {
                        type Response = super::QueryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearCacheMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).clear_cache(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClearCacheSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/all.BigBaller/Challenge" => {
                    #[allow(non_camel_case_types)]
                    struct ChallengeSvc<T: BigBaller>(pub Arc<T>);
                    impl<T: BigBaller> tonic::server::UnaryService<super::Token>
                    for ChallengeSvc<T> {
                        type Response = super::QueryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Token>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).challenge(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChallengeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/all.BigBaller/ChallengeRemoval" => {
                    #[allow(non_camel_case_types)]
                    struct ChallengeRemovalSvc<T: BigBaller>(pub Arc<T>);
                    impl<T: BigBaller> tonic::server::UnaryService<super::Token>
                    for ChallengeRemovalSvc<T> {
                        type Response = super::QueryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Token>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).challenge_removal(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChallengeRemovalSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/all.BigBaller/ChallengeCompleted" => {
                    #[allow(non_camel_case_types)]
                    struct ChallengeCompletedSvc<T: BigBaller>(pub Arc<T>);
                    impl<T: BigBaller> tonic::server::UnaryService<super::Cert>
                    for ChallengeCompletedSvc<T> {
                        type Response = super::QueryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Cert>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).challenge_completed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChallengeCompletedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/all.BigBaller/RefreshChallenge" => {
                    #[allow(non_camel_case_types)]
                    struct RefreshChallengeSvc<T: BigBaller>(pub Arc<T>);
                    impl<
                        T: BigBaller,
                    > tonic::server::UnaryService<super::SmartChallengeScript>
                    for RefreshChallengeSvc<T> {
                        type Response = super::QueryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SmartChallengeScript>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).refresh_challenge(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RefreshChallengeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: BigBaller> Clone for BigBallerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: BigBaller> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BigBaller> tonic::server::NamedService for BigBallerServer<T> {
        const NAME: &'static str = "all.BigBaller";
    }
}
