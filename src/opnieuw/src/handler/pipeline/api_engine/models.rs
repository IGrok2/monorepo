/*
API ENGINE
users delegate a path like:
    - api.diamondcdn.com/*
    - diamondcdn.com/api/*

which will be TOTALLY managed by the api engine, a firewall with allow rules but otherwise denies traffic.

Users inform us of the specifications of their APIs and we import their schemas either manually, or via schemas such as OpenAPI or Swagger

We then take that and enforce the rules. Paths are to be directly matched. We only allow certain methods to these paths, and allow users to tag ratelimiting buckets too!

TODO: coming soon: authenticated endpoints & enforcing certain schemas
 */
 */
 */

/*
we have allow / deny rules then we have rules that change certain behaviors,

it can't be as strict as allow and deny for EVERY path with no methods

perhaps methods are part of a more general rule, while exact paths just have to be listed?
 */

use crate::buckets::models::PublicBucket;
use crate::handler::pipeline::caching::models::CacheLevel;
use crate::utils::counter::Counter;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct ApiEngineSettings {
    pub enabled: bool,
    pub strict_mode: bool, // blocks all non-get requests that aren't going to the api engine
    pub rules: Vec<Setting>,
}

#[derive(Clone, Debug)]
pub struct Setting {
    // for each "setting"
    pub host_path: String,
    pub whitelist_factors: Vec<WhitelistFactors>, // allow these to pass no matter what
    pub rules: Vec<Rule>,
    pub open_api: bool, // allows bots to access this endpoint. TODO: implement in gRPC
}

#[derive(Clone, Debug)]
pub enum WhitelistFactors {
    Ip(String),
    Header(String, String),
}

#[derive(Clone, Debug)]
pub struct Rule {
    // if it's here, it's allowed
    pub id: String,
    pub position: u16,
    pub hit: Counter,
    pub path: Vec<String>, // one thing to note is that this path MUST CONTAIN THE SETTING PATH!!!!!!!!!!!!!!!!!!!!!!!!!!! TODO: ENSURE THIS IS DONE ON THE API TODO: implement *
    pub trigger_type: TriggerType,
    pub allow_query_string: bool, // allow a query string to pass? TODO: implement
    pub methods_allowed: Vec<Method>,
    pub action: Vec<Action>,
    pub human_engine_validation: bool,
}

#[derive(Clone, Debug)]
pub enum TriggerType {
    UseStar, // just use the star
    Exact,
    Contains,
    StartsWith,
}

#[derive(Clone, Debug)]
pub enum Method {
    Web(Vec<WebMethods>),
    Ws(Vec<WsMethods>),
}

#[derive(Clone, Debug)]
pub enum WebMethods {
    Get,
    Post,
    Options,
    Delete,
    Head,
    Put,
    Connect,
    Trace,
    Patch,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum WsMethods {
    Ping,
    Txt,
    Binary,
    Close,
}

#[derive(Clone, Debug)]
pub enum Action {
    Ratelimit(Arc<PublicBucket>),
    Cache(CacheLevel, Option<Duration>),
}
