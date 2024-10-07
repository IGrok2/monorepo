use crate::{
    handler::pipeline::{
        caching::models::{
            CacheLevel,
            CachedObject,
        },
        rules::models::Trustbusting::Cache,
    },
    models::request_context::{
        PipelineData,
        RequestContext,
    },
};
use actix_web::{
    http::header::HeaderValue,
    web,
    HttpMessage,
    HttpResponse,
    HttpResponseBuilder,
};
use awc::{
    error::PayloadError,
    http::{
        header::{
            HeaderMap,
            CACHE_CONTROL,
        },
        StatusCode,
    },
    ClientResponse,
    ResponseBody,
};
use bytes::Bytes;
use dashmap::{
    mapref::one::Ref,
    DashMap,
};
use futures_util::{
    Stream,
    StreamExt,
    TryStreamExt,
};
use sled::{
    Batch,
    IVec,
};
use std::{
    pin::Pin,
    str::FromStr,
    sync::Arc,
    time::{
        Duration,
        Instant,
    },
};
use tokio::task;

lazy_static! {
    static ref CACHE_INFOS_DB: DashMap<String, bool> = DashMap::new();
}

pub async fn cache_handler(
    ctx: &RequestContext,
    resp: &mut ClientResponse,
    data: &[PipelineData],
) -> Option<bool> {
    // return type not used, but used to skip the program & debugging
    if !ctx.domain.caching_settings.enabled || !ctx.domain.internal_settings.can_cache {
        return None;
    }

    let mut level = CacheLevel::Null;
    let mut ttl: Option<Duration> = None;
    let mut transform = false;

    for i in data {
        if let Some(t) = i.cache_level {
            if t == CacheLevel::Null {
                return None;
            } else {
                level = t;
                match t {
                    // derive the ttl from the cachelevel
                    CacheLevel::Standard(t)
                    | CacheLevel::Aggressive(t)
                    | CacheLevel::IgnoreQueryString(t) => {
                        ttl = t; // we want this to override whatever the headers tell us to do
                    }
                    _ => {} // don't do anything
                }
            }
        }
    }

    if level == CacheLevel::Null {
        if let Some(t) = check_cache_headers(resp.headers()) {
            if t.0 .0 == true {
                if let Some(i) = t.0 .1 {
                    ttl = Some(Duration::from_secs(i))
                }
            } else {
                return None; // it shouldn't be cached
            }
        } else {
            return None; // the header didn't even exist
        }
    }

    if ttl.is_none() {
        // if there's still no time to live after checking the headers, rely on the default
        // here we don't want to override and instead respect headers, but if we can't then we have to do this
        match ctx.domain.caching_settings.default_cache_level {
            // derive the ttl from the cachelevel
            CacheLevel::Standard(t)
            | CacheLevel::Aggressive(t)
            | CacheLevel::IgnoreQueryString(t) => {
                ttl = t;
            }
            _ => {} // don't do anything
        }
    }

    let key = ctx.full_host.clone() + ctx.req.path();

    match ctx.domain.caching_settings.cached_entries.get(&key) {
        Some(t) => {
            if t.level == level {
                // worried about expired entries? worry not, that shouldn't be done here anyways
                return None; // this has already been cached
            }
        }
        None => {} // good, nothing
    }

    match CACHE_INFOS_DB.get(&key) {
        Some(t) => {}
        None => return None, // resource is currently being cached, leave it alone
    }

    CACHE_INFOS_DB.insert(key.clone(), true);

    let body = match resp
        .body()
        .limit(ctx.domain.internal_settings.cache_file_max)
        .await
    {
        Ok(t) => t,
        Err(_) => return None,
    };

    let val = ctx
        .domain
        .caching_settings
        .cached_entries
        .insert(
            key.clone(),
            Arc::new(CachedObject {
                root_domain: ctx.domain.domain.clone(),
                id: key.clone(),
                size: body.len() as u64, // assume the worst
                created_at: Instant::now(),
                lasts: ttl.unwrap(), // we would want to propagate an error here
                status: resp.status(),
                headers: resp.headers().clone(),
                level,
            }),
        )
        .unwrap(); // an error here would be bad, best to propagate it

    ctx.domain
        .analytic
        .cached_data
        .inc_by((body.len() / 1000) as u32);

    val.put(IVec::from(body.to_vec()));

    CACHE_INFOS_DB.remove(&key);

    Some(true)
}
/*
    task::spawn(do_it(
        resp.body().limit(ctx.domain.internal_settings.cache_file_max as usize),
        ctx.domain.domain.clone(),
        ctx.domain.caching_settings.cached_entries,
        key,
        ttl,
        level,
        resp.status(),
        resp.headers().clone()
    ));

async fn do_it(unawaited_body: ResponseBody<Pin<Box<dyn Stream<Item = Result<Bytes, PayloadError>>>>>, domain: String, cached_entries: &DashMap<String, Arc<CachedObject>>,
                  key: String, ttl: Option<Duration>, level: CacheLevel, status: StatusCode, headers: HeaderMap) -> bool {
    async move {
        let body = match unawaited_body.await {
            Ok(t) => t,
            Err(_) => return false
        };

        let val = cached_entries.insert(key.clone(), Arc::new(CachedObject {
            root_domain: domain,
            id: key,
            size: body.len() as u64, // assume the worst
            created_at: Instant::now(),
            lasts: ttl.unwrap(), // we would want to propagate an error here
            status,
            headers,
            level,
        })).unwrap(); // an error here would be bad, best to propagate it

        val.put(IVec::from(body.to_vec()));

        true
    };
    true
}*/

fn check_cache_headers(headers: &HeaderMap) -> Option<((bool, Option<u64>), bool)> {
    // should be cached, the ttl, and if it should be transformed
    let mut resp = false;
    let mut ttl: Option<u64> = None;
    let mut should_transform = true;

    match headers.get(CACHE_CONTROL) {
        Some(t) => {
            let val = t.to_str().unwrap();
            if val.contains("public") {
                // public means that we can just serve the cache with a max-age
                if val.contains("max-age=") {
                    ttl = val
                        .split(", ")
                        .filter(|val| val.starts_with("max-age=")) // get the actual value
                        .next()
                        .map(|max_age| {
                            u64::from_str(max_age.trim_start_matches("max-age=")).unwrap_or(0)
                        });
                }
            }
            if val.contains("no-transform") {
                should_transform = false;
            }
        }
        None => return None,
    }

    Some(((resp, ttl), should_transform));
    Some(((true, Some(60)), false))
}
