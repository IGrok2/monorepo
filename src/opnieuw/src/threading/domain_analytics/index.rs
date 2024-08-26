use clickhouse::Client;
use std::time::Duration;

use clickhouse::Row;
use serde::{
    Deserialize,
    Serialize,
};
use time::OffsetDateTime;
use tokio::time::timeout;

use crate::{
    buckets::private::PrivateKeys,
    debug,
    models::domain_context::OriginType,
    templates::error::internal_error,
    DOMAINS_DB,
    GA,
};

//default
//T8O.rORW5Y6u0

lazy_static! {
    static ref CLICKHOUSE_HOST: String =
        "http://".to_string() + &std::env::var("CLICKHOUSE_HOST").unwrap();
    static ref CLICKHOUSE_PASSWORD: String = std::env::var("CLICKHOUSE_PASSWORD").unwrap();
    static ref DB: Client = Client::default()
        .with_url(CLICKHOUSE_HOST.as_str())
        .with_user("default")
        .with_password(CLICKHOUSE_PASSWORD.as_str())
        .with_database("domain-analytics");
}

#[derive(Debug, Row, Serialize, Deserialize)]
struct ClickhouseAnalyticRow {
    #[serde(with = "clickhouse::serde::time::datetime")]
    date: OffsetDateTime,
    region: String,

    // STANDARD ANALYTICS
    total: u32,

    // security stuff
    blocked_reqs: u32,
    smart_challenged_reqs: u32,
    api_engine_blocked: u32,
    ratelimited: u32,
    rl_backend: u32,

    // other stuff
    proxied_reqs: u32,
    cached_reqs: u32,
    cookie_verified: u32,

    // errors
    invalid_reqs: u32,
    origin_err_reqs: u32,

    // data transferred (passed only)
    data_transferred: u32,

    // RATELIMITING BUCKETS
    buckets: Vec<(String, (u32, u32))>,

    // PAGE RULES
    rules: Vec<(String, u32)>,

    // API ENGINE RULE
    api_engine: Vec<(String, u32)>,

    // ORIGIN ANALYTICS
    #[allow(clippy::type_complexity)]
    origin_analytics: Vec<(String, ((u32, u32), Vec<(String, u32)>))>,

    // PRIVATE ANALYTICS
    private_analytics: Vec<(String, (u32, u32))>,
    bots: Vec<(String, (u32, u32))>, // first is bot id, second is passed, third is ratelimited,

    cache_store: u32, // bytes / 1000

    // data transferred by the cache
    data_transferred_cache: u32,

    // turbo mode
    turbo_served: u32,
    turbo_confirmed: u32,

    // challenge
    challenge_completed: u32,
}

fn passed_ratelimited_private(count: u32, threshold: u32) -> (u32, u32) {
    match count {
        _ if count > threshold => (threshold, count - threshold),
        _ if threshold > count => (count, 0),
        _ => (threshold, 0),
    }
}

pub async fn do_domain_analytics() {
    let mut vec = Vec::new();

    debug!("do domain analytics");

    for n in DOMAINS_DB.iter() {
        let mut to_remove_paths = Vec::new();

        for i in n.human_engine_settings.internal_bucket.paths.iter() {
            if i.get() > n.internal_settings.expected_passed as i64 {
                i.dec_by(n.internal_settings.expected_passed as i64);
            } else {
                to_remove_paths.push(i.key().clone())
            }
        }

        // passed human engine reset
        n.analytic.passed_human_engine.get_and_reset();

        for i in to_remove_paths {
            n.human_engine_settings.internal_bucket.paths.remove(&i);
        }

        debug!("Domains db vector pushed");

        vec.push(n.value().clone())
    }

    // iterate through all the DomainContexts
    for n in vec {
        debug!("Domains db checking");

        // clear any expired cache
        n.clear_cache();

        // get the examples in the database
        n.do_analytics_by_example().await;

        // remove the examples from the DashMap
        n.analytics_by_example.examples.remove(&0);

        if n.analytic.total.get() == 0 {
            continue;
        }

        // clone it so we can have full access
        let i = n.analytic.clone();

        let mut private_analytics = Vec::new();
        let mut bots = Vec::new();

        debug!("reading from map ...");

        {
            let read = n.private_bucket.map.read().unwrap();

            for (k, v) in read.iter() {
                let (passed, threshold) =
                    passed_ratelimited_private(v.count.get_and_reset() as u32, v.threshold);

                match k {
                    PrivateKeys::BotKey(x) => {
                        bots.push((format!("{:?}", x), (passed, threshold)));
                    }
                    _ => {
                        private_analytics.push((format!("{:?}", k), (passed, threshold)));
                    }
                }
            }

            drop(read);
        }

        let mut buckets = Vec::new();

        for i in n.buckets.iter() {
            buckets.push((
                i.bucket.name.clone(),
                (
                    i.bucket.passed.get_and_reset() as u32,
                    i.bucket.ratelimited.get_and_reset() as u32,
                ),
            ))
        }

        crate::debug!("buckets: {:?}", buckets);

        let mut page_rules = Vec::new();

        for i in n.rules.rules.iter() {
            crate::debug!("page rule {}: {:?}", i.id, i.analytic);

            page_rules.push((i.id.clone(), i.analytic.get_and_reset() as u32));
        }

        let mut api_engine_rules = Vec::new();

        for i in n.api_engine_settings.rules.iter() {
            for n in i.rules.iter() {
                crate::debug!("api engine rule {}: {:?}", n.id, n.hit);

                api_engine_rules.push((n.id.clone(), n.hit.get_and_reset() as u32))
            }
        }

        let mut origin_analytics = Vec::new();

        if let Some(app) = &n.app_settings {
            // app origin analytics are different

            let i = app.origin_settings.clone();

            origin_analytics.push((
                "app".to_string(),
                (
                    (
                        i.hits.get_and_reset() as u32,
                        i.errored.get_and_reset() as u32,
                    ),
                    Vec::new(),
                ),
            ));
        } else {
            for v in n.origin.settings.iter() {
                crate::debug!("origin {}: {:?}", v.key(), v.value());

                match v.value().as_ref() {
                    OriginType::App(_) => {
                        continue;
                    }
                    OriginType::Origin(i) => {
                        let mut specific_origin_value = Vec::new();

                        for n in i.origins.iter() {
                            if n.1 .0 .1.get() == 0 {
                                continue;
                            }

                            specific_origin_value
                                .push((n.0.to_string(), n.1 .0 .1.get_and_reset() as u32))
                        }

                        origin_analytics.push((
                            v.key().clone(),
                            (
                                (
                                    i.hits.get_and_reset() as u32,
                                    i.errored.get_and_reset() as u32,
                                ),
                                specific_origin_value,
                            ),
                        ));
                    }
                }
            }
        }

        let key = format!("`domain-analytics`.`{}`", n.domain.clone());

        let cache_store = n.caching_settings.bucket.total_size.get() / 1000;

        drop(n);

        let mut insert = DB.insert(&key).unwrap(); // get insert statement ready

        debug!("insert start for key {key}");

        if let Err(_e) = timeout(
            Duration::from_secs(2),
            insert.write(&ClickhouseAnalyticRow {
                date: OffsetDateTime::now_utc(),
                region: std::env::var("REGION").unwrap(),
                total: i.total.get_and_reset() as u32,
                blocked_reqs: i.blocked_reqs.get_and_reset() as u32,
                smart_challenged_reqs: i.smart_challenged_reqs.get_and_reset() as u32,
                api_engine_blocked: i.api_engine_blocked.get_and_reset() as u32,
                ratelimited: i.ratelimited.get_and_reset() as u32,
                rl_backend: i.internally_ratelimited.get_and_reset() as u32,
                proxied_reqs: i.proxied_reqs.get_and_reset() as u32,
                cached_reqs: i.cached_reqs.get_and_reset() as u32,
                cookie_verified: i.cookie_verified.get_and_reset() as u32,
                invalid_reqs: i.invalid_reqs.get_and_reset() as u32,
                origin_err_reqs: i.origin_err_reqs.get_and_reset() as u32,
                data_transferred: i.data_transferred_outbound.get_and_reset() as u32
                    + i.data_transferred_inbound.get_and_reset() as u32,
                turbo_served: i.turbo_mode_served.get_and_reset() as u32,
                turbo_confirmed: i.turbo_mode_completed.get_and_reset() as u32,
                challenge_completed: i.challenge_completed.get_and_reset() as u32,
                buckets,
                rules: page_rules,
                api_engine: api_engine_rules,
                origin_analytics,
                private_analytics,
                bots,
                cache_store: cache_store as u32,
                data_transferred_cache: i.data_transferred_cache.get_and_reset() as u32,
            }),
        )
        .await
        {
            // internal error
            internal_error(&format!(
                "Writing to analytics timed out for domain {}",
                key
            ));
        };

        debug!("insert end");

        match timeout(Duration::from_secs(2), insert.end()).await {
            Ok(v) => match v {
                Ok(_t) => {
                    GA.threading.rows_inserted.inc();
                    debug!("inserted!");
                }
                Err(t) => {
                    GA.threading.rows_errored.inc();
                    debug!("\nTHE ERROR: {:?}\n", t);
                    /*

                    // RATELIMITING BUCKETS

                    buckets: Vec<(String, (i32, i32))>,

                    // PAGE RULES

                    rules: Vec<(String, i32)>,

                    // API ENGINE RULE

                    api_engine: Vec<(String, i32)>,

                    // ORIGIN ANALYTICS

                    origin_analytics: Vec<(String, ((i32, i32), Vec<(String, i32)>))>,

                    // PRIVATE ANALYTICS

                    private_analytics: Vec<(String, (i32, i32))>,
                    bots: Vec<(String, (i32, i32))>, // first is bot id, second is passed, third is ratelimited
                                 */
                    DB.query(&format!(r##"CREATE TABLE {} (
                    // the basic analytics
                    date DateTime,
                    region String,
                    total Int32,
                    blocked_reqs Int32,
                    smart_challenged_reqs Int32,
                    api_engine_blocked Int32,
                    rl_backend Int32,
                    ratelimited Int32,
                    cookie_verified Int32,
                    proxied_reqs Int32,
                    cached_reqs Int32,
                    invalid_reqs Int32,
                    origin_err_reqs Int32,
                    data_transferred Int32,

                    // ratelimiting buckets
                    buckets Array(Tuple(String, Tuple(Int32, Int32))), // bucket name, allowed, ratelimited

                    // page rules
                    rules Array(Tuple(String, Int32)), // id, times hit

                    api_engine Array(Tuple(String, Int32)),

                    origin_analytics Array(Tuple(String, Tuple(Tuple(Int32, Int32), Array(Tuple(String, Int32))))), // host, setting hits, urls & url hits

                    private_analytics Array(Tuple(String, Tuple(Int32, Int32))), // setting, passed, ratelimited
                    bots Array(Tuple(String, Tuple(Int32, Int32))),

                    cache_store Int32, // bytes / 1000

                    // data transferred by the cache
                    data_transferred_cache Int32,

                    turbo_served Int32,
                    turbo_confirmed Int32,
                    challenge_completed Int32
                ) ENGINE = MergeTree()
                TTL date + INTERVAL 7 DAY
                ORDER BY date;"##, key)).execute().await.unwrap_or_else(|err2| {
                        internal_error(&format!("Errored twice whilst inserting: {:?}, second: {:?}", t, err2));
                    });
                }
            },
            Err(_e) => GA.threading.rows_timedout.inc(),
        };
    }
}
