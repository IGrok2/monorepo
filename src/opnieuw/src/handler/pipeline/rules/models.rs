use crate::buckets::models::PublicBucket;
use crate::handler::pipeline::caching::models::CacheLevel;
use crate::utils::counter::Counter;
use hyper::Method;
use std::sync::Arc;
use std::time::Duration;

/* 3: Rules!
   - Rules allows users to control traffic not from APIs. It acts on triggers and can perform actions.
       - Triggers - can either be a part of a string (beginning of path) or an entire one.
           Can match one triggers, two triggers, one trigger and not the other, etc:
               - IP address
               - Path
               - Query string
               - ASN
               - Country
               - Headers
       - Actions:
           - Captcha
           - SMART challenge
           - Block
           - Caching along with caching level
           - Redirections (and can even utilize parameters, such as redirect to diamondcdn.com/country/{country})
           - Utilize ratelimiting bucket
*/

#[derive(Clone, Debug)]
pub struct Rule {
    pub id: String,
    pub trigger: Trigger,
    pub action: Action,
    pub analytic: Counter,
    pub enabled: bool,
    pub max: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct Trigger {
    pub match_type: Vec<Match>,
    pub trigger_requirement: TriggerRequirement,
    pub inversed: bool, // if the entire rule is inversed
}

#[derive(Clone, Eq, Debug, PartialEq)]
pub enum TriggerRequirement {
    One,           // only one needs to match
    All,           // all of them need to match
    Multiple(u32), // a specific amount of them need to match
}

#[derive(Clone, Debug)]
pub struct Match {
    pub trigger: TriggerType,
    pub m_type: MatchType,
    pub inversed: bool, // if just this match is inversed
    pub required: bool, // if this is required
}

#[derive(Clone, Debug)]
pub enum MatchType {
    UseStar,
    Exact,
    Contains,
    StartsWith,
}

#[derive(Clone, Debug)]
pub enum TriggerType {
    Proto(String),
    Ip(String),
    Path(String),
    Query { key: String, value: String }, // key "*" means it works for all queries, value "*" means it works for all queries under that key
    Asn(String),
    Country(String),
    Continent(String),
    Headers { key: String, value: String }, // works the same as the query,
    Host(String),
    Method(Method),
    UserAgent(String),
    Cookie(String),
    Any, // TODO: Websocket shit
}

#[derive(Clone, Debug)]
pub enum Action {
    Monopoly(Monopoly),
    Trustbusting(Vec<Trustbusting>),
}

#[derive(Clone, Debug)]
pub enum Monopoly {
    // if this action happens, then no others can happen
    Block,
}

#[derive(Clone, Debug)]
pub enum Trustbusting {
    // multiple of these actions can occur at the same time
    SmartChallenge, // TODO: make it so only one challenge can be enabled at a time
    CaptchaChallenge,
    SkipHumanEngine,
    Rewrite(String),
    RatelimitBucket(Arc<PublicBucket>),  // name of the bucket
    Cache(CacheLevel, Option<Duration>), // TODO
    Redirect(String),                    // where to redirect to
    UseBackend(String),                  // the host of the backend we will be using
    UseApp(String),                      // the app url
}
