use crate::utils::counter::Counter;

// threading will store the threading per zone. it shouldn't use any pointers (which means no lifetimes)
// threading are atomics
#[derive(Debug)]
pub struct Analytic {
    // EVERYTHING combined is the TOTAL REQUEST COUNTER for the zone
    pub total: Counter,

    // security counters, added up these are SECURITY EVENTS
    // requests blocked by rules or anything else
    pub blocked_reqs: Counter,
    // all requests that are challenged by our system
    pub smart_challenged_reqs: Counter,
    // api engine blocked requests for not matching anything
    pub api_engine_blocked: Counter,
    // ratelimited requests as a whole
    pub ratelimited: Counter,
    // waf blocked this request
    pub waf_block: Counter,

    // combined, these are the security events for the zone

    // all requests that are proxied to the users' backend, added up these are PASSED REQUESTS
    pub proxied_reqs: Counter,
    // all requests answered by our cache
    pub cached_reqs: Counter,
    // requests where the cookie has been validated
    pub cookie_verified: Counter,
    // total websocket messages from clients
    pub websocket_client_messages: Counter,

    // these are requests that cannot completed not necessarily because they are bad, but because we can't serve them
    // we experienced an error!
    pub invalid_reqs: Counter,
    // the backend returned an error or we flat out couldn't connect
    pub origin_err_reqs: Counter,

    // the KB's transferred either by cache or by backend (for backend, count twice)
    // only clean traffic
    pub data_transferred_outbound: Counter,
    pub data_transferred_inbound: Counter,
    pub data_transferred_cache: Counter,

    // when streams were allowed vs when they weren't
    pub allowed_streams: Counter,
    pub blocked_streams: Counter,

    // INTERNAL METRICS
    pub passed_human_engine: Counter,
    // AVERAGE THREAT SCORE (before the last counter)
    pub ts_total_reqs: Counter,
    // the threat score
    pub ts_total_counter: Counter,
    // how much data is being cached, in KB
    pub cached_data: Counter,
    // current connections sending data to the server
    pub sending_data: Counter,
    // the amount of times this domain has been internally ratelimited
    pub internally_ratelimited: Counter,

    // turbo mode & challenge modes
    pub challenge_completed: Counter,

    // turbo mode specific
    pub turbo_mode_served: Counter,
    pub turbo_mode_completed: Counter,
}

// only one these objects should exist, used to metric what is happening internally
// does not repeat what happens to domains! only errors and such
#[derive(Debug)]
pub struct GlobalAnalytics {
    // total requests encountered
    pub total_reqs: Counter,
    // requests that went direct to IP
    pub ip_reqs: Counter,
    // requests that we encountered really weird errors for
    pub foo_errs: Counter,
    // requests where the domain wasn't found
    pub domain_not_found: Counter,
    // requests that are globally ratelimited
    pub globally_ratelimited: Counter,
    // requests that are to a domain which is suspended
    pub suspended_reqs: Counter,
    // requests that are going to the CGI paths
    pub cgi_reqs: Counter,
    // threat score threading
    pub ts: ThreatScoreAnalytics,
}

#[derive(Debug)]
pub struct ThreatScoreAnalytics {
    // important to have threat score threading so we can detect bypasses quickly
    // everything is done by score allocated
    pub headers: Counter,
    pub tls: Counter,
    pub user_agent: Counter,
    pub open_proxy: Counter,
    pub domain_traffic: Counter,
}

impl Analytic {
    pub fn new() -> Self {
        Self {
            total: Counter::new(),
            blocked_reqs: Counter::new(),
            smart_challenged_reqs: Counter::new(),
            api_engine_blocked: Counter::new(),
            ratelimited: Counter::new(),
            waf_block: Counter::new(),
            proxied_reqs: Counter::new(),
            cached_reqs: Counter::new(),
            cookie_verified: Counter::new(),
            websocket_client_messages: Counter::new(),
            invalid_reqs: Counter::new(),
            origin_err_reqs: Counter::new(),
            data_transferred_outbound: Counter::new(),
            data_transferred_inbound: Counter::new(),
            data_transferred_cache: Counter::new(),
            allowed_streams: Counter::new(),
            blocked_streams: Counter::new(),
            passed_human_engine: Counter::new(),
            ts_total_reqs: Counter::new(),
            ts_total_counter: Counter::new(),
            cached_data: Counter::new(),
            sending_data: Counter::new(),
            internally_ratelimited: Counter::new(),
            turbo_mode_completed: Counter::new(),
            turbo_mode_served: Counter::new(),
            challenge_completed: Counter::new(),
        }
    }
}
