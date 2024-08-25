use crate::buckets::models::PublicBucket;
use crate::models::domain_context::DomainContext;
use crate::models::request_context::RequestContext;
use crate::tls::models::TlsFingerprint;
use crate::utils::counter::Counter;
use clickhouse::Row;
use dashmap::DashMap;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct AnalyticsByExample {
    pub timestamp: Instant,
    pub domain: Arc<DomainContext>,
    pub general: GeneralByExample,
    pub api_engine: ApiEngineByExample,
    pub bot_management: BotManagementByExample,
    pub caching: CachingByExample,
    pub human_engine: HumanEngineByExample,
    pub rules: RulesByExample,
    pub proxy: ProxyByExample,
    pub domain_stats: DomainStats,
}

#[derive(Clone, Debug, Row, Serialize, Deserialize)]
pub struct GeneralByExample {
    pub ip: String,
    pub host: String,
    pub method: String,
    pub user_agent: String,
    pub path: String,
    pub elapsed: Option<Duration>,
    pub country: String,
    pub asn: String,
    pub fingerprint: Option<TlsFingerprint>,
    pub referrer: Option<String>,
}

#[derive(Clone, Debug, Row, Serialize, Deserialize)]
pub struct DomainStats {
    pub domain_traffic: i64,
    pub open_conns: i64,
    pub cache_store: i64,
}

#[derive(Clone, Debug, Row, Serialize, Deserialize)]
pub struct ApiEngineByExample {
    pub done: bool,
    pub setting: String,
    pub rule: String,
    pub actions: Vec<String>,
}

#[derive(Clone, Debug, Row, Serialize, Deserialize)]
pub struct BotManagementByExample {
    pub allowed: bool,
}

#[derive(Clone, Debug, Row, Serialize, Deserialize)]
pub struct CachingByExample {
    pub hit: bool,
    pub encoded: bool,
    pub bytes_written: u64,
}

#[derive(Clone, Debug, Row, Serialize, Deserialize)]
pub struct HumanEngineByExample {
    pub hit: bool,
    pub threat_score: u32,
    pub smart_challenge_served: bool,
    pub turbo_mode_served: bool,
    pub cookie_validated: bool,
}

#[derive(Clone, Debug, Row, Serialize, Deserialize)]
pub struct RulesByExample {
    pub hit: bool,
    pub rule_ids: Vec<String>,
    pub action: Vec<String>,
}

#[derive(Clone, Debug, Row, Serialize, Deserialize)]
pub struct ProxyByExample {
    pub hit: bool,
    pub errored: Option<String>,
    pub response_code: Option<u16>,
    pub origin_setting_ip: String,
    pub bytes_written: u64,
}

#[derive(Clone, Debug)]
pub struct AnalyticsByExampleDomain {
    pub started: Counter,
    pub examples: DashMap<u8, Vec<AnalyticsByExampleDone>>,
}

#[derive(Clone, Debug)]
// analytics by example middleware
pub struct AnalyticsByExampleDone {
    pub timestamp: Instant,
    pub domain: Arc<DomainContext>,
    pub general: GeneralByExample,
    pub api_engine: ApiEngineByExample,
    pub bot_management: BotManagementByExample,
    pub caching: CachingByExample,
    pub human_engine: HumanEngineByExample,
    pub rules: RulesByExample,
    pub proxy: ProxyByExample,
    pub domain_stats: DomainStats,
}

impl AnalyticsByExample {
    pub fn new(request_ctx: &RequestContext) -> Option<AnalyticsByExample> {
        // determine if we should collect metrics
        let analytics_started = request_ctx.domain.analytics_by_example.started.get();

        if analytics_started < 20 {
            // 20 allowed per 10s

            let distance_to_started = 20 - analytics_started;

            // TODO: start using again by removing the zero
            if thread_rng().gen::<f32>() < distance_to_started as f32 * 0.01 {
                // okay, it's going to be here

                // get the ip data and the tls fingerprint
                let ip_info = request_ctx.get_ipdata();
                let fingerprint = request_ctx.connection_context.fingerprint.clone();

                // get the referrer
                let referrer = match request_ctx.req.headers.get("referer") {
                    Some(t) => match t.to_str() {
                        Ok(t) => Some(t.to_string()),
                        Err(_) => None,
                    },
                    None => None,
                };

                return Some(AnalyticsByExample {
                    domain: request_ctx.domain.clone(),
                    timestamp: Instant::now(),
                    general: GeneralByExample {
                        ip: request_ctx.ip.ip.clone(),
                        host: format!("{:?}", request_ctx.req.uri.host()).clone(),
                        method: request_ctx.req.method.to_string(),
                        user_agent: request_ctx.user_agent.clone(),
                        path: request_ctx.req.uri.path().to_string(),
                        elapsed: None,
                        country: ip_info.0,
                        asn: ip_info.2,
                        fingerprint,
                        referrer,
                    },
                    api_engine: ApiEngineByExample {
                        done: false,
                        setting: "".to_string(),
                        rule: "".to_string(),
                        actions: vec![],
                    },
                    bot_management: BotManagementByExample { allowed: false },
                    caching: CachingByExample {
                        hit: false,
                        encoded: false,
                        bytes_written: 0,
                    },
                    human_engine: HumanEngineByExample {
                        hit: false,
                        threat_score: 0,
                        smart_challenge_served: false,
                        turbo_mode_served: false,
                        cookie_validated: false,
                    },
                    rules: RulesByExample {
                        hit: false,
                        rule_ids: vec![],
                        action: vec![],
                    },
                    proxy: ProxyByExample {
                        hit: false,
                        errored: None,
                        response_code: None,
                        origin_setting_ip: "".to_string(),
                        bytes_written: 0,
                    },
                    domain_stats: DomainStats {
                        domain_traffic: request_ctx.domain.analytic.total.get(),
                        open_conns: request_ctx.domain.origin.open_conns.get(),
                        cache_store: request_ctx.domain.caching_settings.bucket.total_size.get(),
                    },
                });
            }
        }

        None
    }

    pub fn write(&mut self) {
        // how long the capture was for
        self.general
            .elapsed
            .replace(Instant::now() - self.timestamp);

        if let Some(mut t) = self.domain.analytics_by_example.examples.get_mut(&0) {
            t.push(AnalyticsByExampleDone {
                timestamp: self.timestamp,
                domain: self.domain.clone(),
                general: self.general.clone(),
                api_engine: self.api_engine.clone(),
                bot_management: self.bot_management.clone(),
                caching: self.caching.clone(),
                human_engine: self.human_engine.clone(),
                rules: self.rules.clone(),
                proxy: self.proxy.clone(),
                domain_stats: self.domain_stats.clone(),
            });
        } else {
            self.domain.analytics_by_example.examples.insert(
                0,
                vec![AnalyticsByExampleDone {
                    timestamp: self.timestamp,
                    domain: self.domain.clone(),
                    general: self.general.clone(),
                    api_engine: self.api_engine.clone(),
                    bot_management: self.bot_management.clone(),
                    caching: self.caching.clone(),
                    human_engine: self.human_engine.clone(),
                    rules: self.rules.clone(),
                    proxy: self.proxy.clone(),
                    domain_stats: self.domain_stats.clone(),
                }],
            );
        }

        // take it out of the in-progress
        self.domain.analytics_by_example.started.dec();
    }
}

impl Drop for AnalyticsByExample {
    fn drop(&mut self) {
        self.write();
    }
}
