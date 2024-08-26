use crate::{
    buckets::private::PrivateKeys::CacheAttempted,
    cache_system::writer::CacheWriter,
    debug,
    handler::pipeline::caching::models::CacheLevel,
    models::{
        domain_context::DomainContext,
        egress_wrapper::EgressWrapper,
        request_context::{
            PipelineData,
            RequestContext,
        },
    },
    rproxy::pipeline::utils::get_content_length,
    GA,
};
use dashmap::DashMap;
use hyper::{
    body::Incoming,
    header::{
        CACHE_CONTROL,
        SET_COOKIE,
    },
    HeaderMap,
    Method,
    Response,
};
use std::{
    str::FromStr,
    sync::Arc,
    time::{
        Duration,
        Instant,
    },
};

impl RequestContext {
    /*
    pub async fn cache_controller(domain: Arc<DomainContext>, resp: &Response<Incoming>, client_resp: &mut HeaderMap, data: &[PipelineData], key: &str) -> Option<CacheWriter> {
        match domain.should_cache(resp, data, key) {
            (CacheLevel::Null, _) | (CacheLevel::None, _) => {
                debug!("not caching");
                None
            }
            (level, mut ttl) => {
                debug!("decided to cache: {:?}", level);

                // this path is about to be cached
                GA.rproxy.attempting_cache.inc();

                client_resp.insert("pw-cache", "attempting".parse().unwrap());

                if ttl.is_none() {
                    ttl = Some(domain.caching_settings.default_cache_ttl)
                }

                CacheWriter::new(domain.clone(),
                    // TODO change TTL back
                    ttl.unwrap(),
                    (self.full_host.to_string() + self.req.uri.path()),
                    resp.headers().clone(),
                    resp.status(),
                    get_content_length(resp)
                ).await
            }
        }
    }

     */

    pub fn should_cache(
        &self,
        resp: &Response<Incoming>,
        data: &[PipelineData],
        key: &str,
    ) -> (CacheLevel, Option<Duration>) {
        // duration is optional if CacheLevel is nothing
        if !self.domain.caching_settings.enabled
            || !self.domain.internal_settings.can_cache
            || self.domain.caching_settings.bucket.total_size.get() as i32
                > self.domain.internal_settings.total_cache_limit
            || self.domain.caching_settings.bucket.map.get(key).is_some()
            || !self.domain.private_bucket.is_allowed(CacheAttempted)
            || get_content_length(resp) / 1000 > self.domain.internal_settings.cache_file_max as u64
            || resp.status() != 200
            || self.req.method != Method::GET
        {
            debug!("failed cache checks");
            // all the checks
            return (CacheLevel::Null, None);
        }

        // if there's a set cookie header, that's also a disqualification
        if resp.headers().get(SET_COOKIE).is_some() {
            debug!("set-cookie header found");
            return (CacheLevel::Null, None);
        }

        for i in data {
            // check our settings
            debug!("iterating through pipeline data ...");
            if let Some(t) = i.cache_level {
                debug!("cache level: {:?}", t.0);
                if t.0 == CacheLevel::Null || t.0 == CacheLevel::None {
                    debug!("pipeline data");
                    return (CacheLevel::Null, None);
                } else {
                    match t.0 {
                        // derive the ttl from the cachelevel
                        CacheLevel::Standard
                        | CacheLevel::Aggressive
                        | CacheLevel::IgnoreQueryString => return (t.0, Some(t.1)),
                        _ => {} // don't do anything
                    }
                    break;
                }
            }
        }

        if self.domain.caching_settings.default_cache_level == CacheLevel::Aggressive {
            return (
                CacheLevel::Aggressive,
                Some(self.domain.caching_settings.default_cache_ttl),
            );
        }

        // ok, so now let's check
        if let Some(t) = check_cache_headers(resp.headers()) {
            debug!("checked cache headers");
            if !t.0 .0 {
                return (CacheLevel::Null, None);
            }

            let mut ttl =
                t.0 .1
                    .unwrap_or(self.domain.caching_settings.default_cache_ttl);

            if ttl.is_zero() {
                ttl = self.domain.caching_settings.default_cache_ttl;
            }

            // ttl on cache setting is not used by default, but instead the caching settings duration is used
            return match self.domain.caching_settings.default_cache_level {
                // map it back around
                CacheLevel::Null => (CacheLevel::Null, None),
                CacheLevel::Standard => (CacheLevel::Standard, Some(ttl)),
                CacheLevel::Aggressive => (CacheLevel::Aggressive, Some(ttl)),
                CacheLevel::None => (CacheLevel::None, None),
                CacheLevel::IgnoreQueryString => (CacheLevel::IgnoreQueryString, Some(ttl)),
            };
        }

        (CacheLevel::Null, None)
    }
}

fn check_cache_headers(headers: &HeaderMap) -> Option<((bool, Option<Duration>), bool)> {
    // should be cached, the ttl, and if it should be transformed
    GA.rproxy.checked_cache_headers.inc();

    let mut resp = false;
    let mut ttl: Option<Duration> = None;
    let mut should_transform = true;

    match headers.get(CACHE_CONTROL) {
        Some(t) => {
            let val = t.to_str().unwrap();
            if val.contains("public") {
                // public means that we can just serve the cache with a max-age
                if val.contains("max-age=") {
                    resp = true;

                    ttl = Some(Duration::from_secs(
                        val.split(", ")
                            .find(|val| val.starts_with("max-age="))
                            .map(|max_age| {
                                u64::from_str(max_age.trim_start_matches("max-age=")).unwrap_or(0)
                            })
                            .unwrap_or(0),
                    ));
                }
            }
            if val.contains("no-transform") {
                should_transform = false;
            }
        }
        None => return None,
    };

    Some(((resp, ttl), should_transform))
}
