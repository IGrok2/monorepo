use crate::buckets::models::{Bucket, PublicBucket};
use crate::buckets::private::PrivateBucket;
use crate::handler::pipeline::api_engine::models::ApiEngineSettings;
use crate::handler::pipeline::human_engine::models::HumanEngine;
use crate::models::analytics_by_example::AnalyticsByExampleDomain;
use crate::models::domain_context::{
    AppSettings, BotManagementSettings, CachingSettings, DomainContext, InternalSettings,
    OriginSettings, RulesSettings,
};
use crate::utils::counter::Counter;
use crate::DOMAINS_DB;
use dashmap::DashMap;
use std::sync::Arc;

pub async fn insert_debugger(
    key: String,
    raw_domain: Arc<DomainContext>,
    origin_settings: Option<OriginSettings>,
    api_engine: Option<ApiEngineSettings>,
    caching_settings: Option<CachingSettings>,
    internal_settings: Option<InternalSettings>,
    bot_settings: Option<BotManagementSettings>,
    human_engine: Option<HumanEngine>,
    buckets: Option<Vec<Arc<PublicBucket>>>,
    private_bucket: Option<Arc<PrivateBucket>>,
    page_rules: Option<RulesSettings>,
    app_settings: Option<Option<Arc<AppSettings>>>,
) {
    DOMAINS_DB.insert(
        key.clone(),
        Arc::new(DomainContext {
            domain: key,
            origin: origin_settings.unwrap_or(raw_domain.origin.clone()),
            api_engine_settings: api_engine.unwrap_or(raw_domain.api_engine_settings.clone()),
            caching_settings: caching_settings.unwrap_or(raw_domain.caching_settings.clone()),
            internal_settings: internal_settings.unwrap_or(raw_domain.internal_settings.clone()),
            bot_settings: bot_settings.unwrap_or(raw_domain.bot_settings.clone()),
            human_engine_settings: human_engine.unwrap_or(raw_domain.human_engine_settings.clone()),
            buckets: buckets.unwrap_or(raw_domain.buckets.clone()),
            private_bucket: private_bucket.unwrap_or(raw_domain.private_bucket.clone()),
            analytic: raw_domain.analytic.clone(),
            rules: page_rules.unwrap_or(raw_domain.rules.clone()),
            analytics_by_example: AnalyticsByExampleDomain {
                started: Counter::new(),
                examples: DashMap::new(),
            },
            app_settings: app_settings.unwrap_or(raw_domain.app_settings.clone()),
        }),
    );
}
