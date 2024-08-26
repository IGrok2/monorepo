use actix_web::http::StatusCode;
use std::time::{
    Duration,
    Instant,
};

/*
#[derive(Debug)]
pub struct CachedObject {
    pub root_domain: String,
    pub id: String, // this should be subdomain.domain.com/{path}
    pub size: u32, // actual amount of bytes divided by 1000
    pub created_at: Instant,
    pub lasts: Duration,
    pub status: StatusCode, // the status from the backend
    pub headers: HeaderMap, // key and value headers
    pub level: CacheLevel
}

 */

// there's a default for the site, then firewall rules can modify it by passing it into additional data vec
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CacheLevel {
    Null,              // there is no cache level set (uses default)
    Standard,          // mindful of query string, caches things with a cache-control header
    Aggressive,        // monkey see, monkey cache
    None,              // no caching :(! use disabled cache header
    IgnoreQueryString, // standard, but forget query string :)
}
