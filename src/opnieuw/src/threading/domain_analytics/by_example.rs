use crate::{
    debug,
    models::{
        analytics_by_example::{
            AnalyticsByExample,
            ApiEngineByExample,
            BotManagementByExample,
            CachingByExample,
            DomainStats,
            GeneralByExample,
            HumanEngineByExample,
            ProxyByExample,
            RulesByExample,
        },
        domain_context::DomainContext,
    },
    GA,
};
use clickhouse::Row;
use serde::{
    Deserialize,
    Serialize,
};
use std::time::Duration;
use time::OffsetDateTime;
use tokio::time::timeout;

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct AnalyticByExampleClickHouse {
    #[serde(with = "clickhouse::serde::time::datetime")]
    pub date: OffsetDateTime,
    // general
    pub ip: String,
    pub host: String,
    pub method: String,
    pub user_agent: String,
    pub path: String,
    pub elapsed: String,
    pub country: String,
    pub asn: String,
    pub fingerprint: String,
    pub referrer: String,
    // api engine
    pub api_hit: bool,
    pub setting: String,
    pub rule: String,
    pub actions: Vec<String>,
    // bot management
    pub bm_allowed: bool,
    // caching
    pub caching_hit: bool,
    pub encoded: bool,
    pub cache_bytes_written: u64,
    // human engine
    pub he_hit: bool,
    pub threat_score: u32,
    pub smart_challenge_served: bool,
    pub turbo_mode_served: bool,
    pub cookie_validated: bool,
    // rules
    pub pr_hit: bool,
    pub rule_ids: Vec<String>,
    pub action: Vec<String>,
    // rproxy
    pub rproxy_hit: bool,
    pub errored: String,
    pub response_code: String,
    pub origin_setting_ip: String,
    pub rproxy_bytes_written: u64,
    // domain info
    pub domain_traffic: u32,
    pub open_conns: u32,
    pub cache_store: u32,
}

use crate::templates::error::internal_error;
use clickhouse::Client;

#[rustfmt::skip]
lazy_static! {
    static ref CLICKHOUSE_HOST: String =
        "http://".to_string() + &std::env::var("CLICKHOUSE_HOST").unwrap();
    static ref CLICKHOUSE_PASSWORD: String = std::env::var("CLICKHOUSE_PASSWORD").unwrap();
    static ref DB: Client = Client::default()
        .with_url(CLICKHOUSE_HOST.as_str())
        .with_user("default")
        .with_password(CLICKHOUSE_PASSWORD.as_str())
        .with_database("by-example");
}

impl DomainContext {
    pub async fn do_analytics_by_example(&self) {
        let mut finished = false;

        if let Some(t) = self.analytics_by_example.examples.get(&0) {
            if !t.is_empty() {
                let key = format!("`by-example`.`{}`", self.domain);

                // begin insertion
                let mut insert = DB.insert(&key).unwrap();

                // iterate through and write
                for val in t.value() {
                    let _ = timeout(
                        Duration::from_secs(2),
                        insert.write(&AnalyticByExampleClickHouse {
                            // date
                            date: OffsetDateTime::now_utc(),
                            // general
                            ip: val.general.ip.clone(),
                            host: val.general.host.clone(),
                            method: val.general.method.clone(),
                            user_agent: val.general.user_agent.clone(),
                            path: val.general.path.clone(),
                            elapsed: format!("{:?}", val.general.elapsed.unwrap()),
                            country: val.general.country.clone(),
                            asn: val.general.asn.clone(),
                            fingerprint: format!("{:?}", val.general.fingerprint),
                            referrer: format!("{:?}", val.general.referrer),
                            // api engine
                            api_hit: val.api_engine.done,
                            setting: val.api_engine.setting.clone(),
                            rule: val.api_engine.rule.clone(),
                            actions: val.api_engine.actions.clone(),
                            // bot management
                            bm_allowed: val.bot_management.allowed,
                            // caching
                            caching_hit: val.caching.hit,
                            encoded: val.caching.encoded,
                            cache_bytes_written: val.caching.bytes_written,
                            // human engine
                            he_hit: val.human_engine.hit,
                            threat_score: val.human_engine.threat_score,
                            smart_challenge_served: val.human_engine.smart_challenge_served,
                            turbo_mode_served: val.human_engine.turbo_mode_served,
                            cookie_validated: val.human_engine.cookie_validated,
                            // page rules
                            pr_hit: val.rules.hit,
                            rule_ids: val.rules.rule_ids.clone(),
                            action: val.rules.action.clone(),
                            // rproxy
                            rproxy_hit: val.proxy.hit,
                            errored: format!("{:?}", val.proxy.errored.clone()),
                            response_code: format!("{:?}", val.proxy.response_code.clone()),
                            origin_setting_ip: val.proxy.origin_setting_ip.clone(),
                            rproxy_bytes_written: val.proxy.bytes_written,
                            // domain stats
                            domain_traffic: val.domain_stats.domain_traffic as u32,
                            open_conns: val.domain_stats.open_conns as u32,
                            cache_store: val.domain_stats.cache_store as u32,
                        }),
                    )
                    .await;
                }

                // finish
                match timeout(Duration::from_secs(2), insert.end()).await {
                    Ok(v) => match v {
                        Ok(_t) => {
                            GA.threading.rows_inserted.inc();
                            debug!("inserted by example!");
                            finished = true;
                        }
                        Err(t) => {
                            GA.threading.rows_errored.inc();
                            debug!("\nTHE ERROR: {:?}\n", t);

                            DB.query(&format!(
                                r##"CREATE TABLE {} (
                    // analytics by example

                    // general
                    date DateTime,
                    ip String,
                    host String,
                    method String,
                    user_agent String,
                    path String,
                    elapsed String,
                    country String,
                    asn String,
                    fingerprint String,
                    referrer String,

                    // api engine
                    api_hit Boolean,
                    setting String,
                    rule String,
                    actions Array(String),

                    // bot management
                    bm_allowed Boolean,

                    // caching
                    caching_hit Boolean,
                    encoded Boolean,
                    cache_bytes_written Int64,

                    // human engine
                    he_hit Boolean,
                    threat_score Int32,
                    smart_challenge_served Boolean,
                    turbo_mode_served Boolean,
                    cookie_validated Boolean,

                    // rules
                    pr_hit Boolean,
                    rule_ids Array(String),
                    action Array(String),

                    // rproxy
                    rproxy_hit Boolean,
                    errored String,
                    response_code String,
                    origin_setting_ip String,
                    rproxy_bytes_written Int64,

                    // domain info
                    domain_traffic Int32,
                    open_conns Int32,
                    cache_store Int32,
                ) ENGINE = MergeTree()
                TTL date + INTERVAL 7 DAY
                ORDER BY date;"##,
                                key
                            ))
                            .execute()
                            .await
                            .unwrap_or_else(|t2| {
                                internal_error(&format!(
                                    "error inserting analytics by example twice: {:?}, {:?}",
                                    t2, t
                                ));
                            });
                        }
                    },
                    Err(_e) => GA.threading.rows_timedout.inc(),
                }
            }
        };

        if finished {
            self.analytics_by_example.examples.clear();
        }
    }
}
