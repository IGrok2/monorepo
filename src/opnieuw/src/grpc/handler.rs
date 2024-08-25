// this is for the server program
use aes::cipher::generic_array::GenericArray;
use aes::cipher::KeyInit;
use aes_gcm::Aes128Gcm;
use bytes::Bytes;
use dashmap::DashMap;
use futures_util::AsyncWriteExt;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

use tonic::{Request, Response, Status};

use crate::buckets::models::PublicBucket;
use crate::buckets::private::PrivateBucket;
use crate::grpc::all::big_baller_server::BigBaller;
use crate::grpc::all::{
    AllDomainSchema, Cert, ClearCacheMessage, DeleteDomainSchema, FullDomainSchema,
    PartialDomainSchema, QueryResponse, SmartChallengeScript, Token,
};
use crate::grpc::tools::{fail, success};
use crate::{
    BACKGROUND_CHALLENGE, CHALLENGE_KEYS, CHALLENGE_RESPONSES, DOMAINS_DB, GA, NEW_DOMAINS_DB,
    SMART_CHALLENGE,
};

use crate::handler::pipeline::api_engine::models::ApiEngineSettings;
use crate::handler::pipeline::bot_management::models::Bots;
use crate::handler::pipeline::caching::models::CacheLevel;
use crate::handler::pipeline::human_engine::models::{
    HumanEngine, HumanEngineMode, InternalCounters,
};
use crate::handler::pipeline::human_engine::scores::failsafe::models::Failsafe;

use crate::models::analytics::Analytic;
use crate::models::domain_context::{
    AppSettings, BotManagementSettings, CachingSettings, DomainContext, InternalSettings,
    OriginSettings, RulesSettings,
};

use crate::utils::counter::Counter;

use crate::debug;
use crate::grpc::insert_debugger::insert_debugger;
use crate::grpc::specific_helpers::RpcToRust;
use crate::models::analytics_by_example::AnalyticsByExampleDomain;
use crate::tls::ssl_models::ChallengeResponse;

#[derive(Default, Clone)]
pub struct FortyTwo {}

#[tonic::async_trait]
impl BigBaller for FortyTwo {
    async fn all_domains(
        &self,
        _request: Request<QueryResponse>,
    ) -> Result<Response<AllDomainSchema>, Status> {
        todo!()
    }

    async fn update_domain(
        &self,
        request: Request<PartialDomainSchema>,
    ) -> Result<Response<QueryResponse>, Status> {
        GA.grpc.update.inc();

        let req = request.into_inner();

        debug!("{:?}", req);

        let mut origin_settings: Option<OriginSettings> = None;
        let mut app_settings: Option<Option<Arc<AppSettings>>> = None;
        let mut api_engine: Option<ApiEngineSettings> = None;
        let mut human_engine: Option<HumanEngine> = None;
        let mut bot_settings: Option<BotManagementSettings> = None;
        let mut caching_settings: Option<CachingSettings> = None;
        let mut buckets: Option<Vec<Arc<PublicBucket>>> = None;
        let mut page_rules: Option<RulesSettings> = None;
        // internal settings and private buckets are two peas in a pod
        let mut internal_settings: Option<InternalSettings> = None;
        let mut private_bucket: Option<Arc<PrivateBucket>> = None;
        /*
                string domain = 1;
        map<string, OriginSetting> origin_settings = 2; // if this isn't part of it, then it'll just be empty
        optional ApiEngineSettings api_engine_settings = 3;
        optional HumanEngine human_engine_settings = 4;
        optional BotSettings bot_settings = 5;
        optional CachingSettings cache_settings = 6;
        optional BucketSettings buckets = 7;
        optional PageRules page_rules = 8;
        optional InternalSettings internal_settings = 9;
               */

        if let Some(domain) = DOMAINS_DB.get(&req.domain) {
            //let domain = domain.value().clone();
            if req.origin_settings.len() > 0 {
                match RpcToRust::origin_settings(req.origin_settings, Some(domain.origin.clone())) {
                    Ok(v) => origin_settings = Some(v),
                    Err(e) => return fail(&e),
                }
            }

            if let Some(apps) = req.app_origin_settings {
                match RpcToRust::app_settings(Some(apps), None) {
                    Ok(v) => app_settings = Some(v),
                    Err(e) => return fail(&e),
                }
            }

            if let Some(ae) = req.api_engine_settings {
                match RpcToRust::api_engine(ae, Some(domain.api_engine_settings.clone())) {
                    Ok(v) => api_engine = Some(v),
                    Err(e) => return fail(&e),
                }
            }

            if let Some(he) = req.human_engine_settings {
                match RpcToRust::human_engine(he, Some(domain.human_engine_settings.clone())) {
                    Ok(v) => human_engine = Some(v),
                    Err(e) => return fail(&e),
                }
            }

            if let Some(b) = req.bot_settings {
                match RpcToRust::bots(b) {
                    Ok(v) => bot_settings = Some(v),
                    Err(e) => return fail(&e),
                }
            }

            if let Some(c) = req.cache_settings {
                match RpcToRust::cache_settings(c, Some(domain.caching_settings.clone())) {
                    Ok(v) => caching_settings = Some(v),
                    Err(e) => return fail(&e),
                }
            }

            if let Some(b) = req.buckets {
                match RpcToRust::bucket_settings(b) {
                    Ok(v) => buckets = Some(v),
                    Err(e) => return fail(&e),
                }
            }

            if let Some(pr) = req.page_rules {
                match RpcToRust::page_rules(
                    pr,
                    Some(domain.rules.clone()),
                    buckets.clone().unwrap_or(domain.buckets.clone()),
                    origin_settings.clone().unwrap_or(domain.origin.clone()),
                ) {
                    Ok(v) => page_rules = Some(v),
                    Err(e) => return fail(&e),
                }
            }

            if let Some(is) = req.internal_settings {
                match RpcToRust::internal_settings(is) {
                    Ok(v) => internal_settings = Some(v),
                    Err(e) => return fail(&e),
                }

                private_bucket = Some(Arc::new(
                    PrivateBucket::new().setup_defaults(&internal_settings.clone().unwrap()),
                ));
            }

            debug!("Inserting into domains.db ... : {}", domain.key().clone());

            let key = domain.key().clone();
            let raw_domain = domain.clone();
            drop(domain);

            debug!("Got domain...");

            timeout(
                Duration::new(1, 0),
                insert_debugger(
                    key.clone(),
                    raw_domain,
                    origin_settings,
                    api_engine,
                    caching_settings,
                    internal_settings,
                    bot_settings,
                    human_engine,
                    buckets,
                    private_bucket,
                    page_rules,
                    app_settings,
                ),
            )
            .await
            .unwrap();

            /*
            DOMAINS_DB.insert(key.clone(), Arc::new(DomainContext {
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
            }));

             */

            debug!("Done!");

            return success();
        }

        return fail("Domain didn't exist!");
    }

    async fn new_domain(
        &self,
        request: Request<FullDomainSchema>,
    ) -> Result<Response<QueryResponse>, Status> {
        GA.grpc.new.inc();

        let req = request.into_inner();
        debug!("request domain: {}", req.domain);
        // appropriate for the free plan

        let insert_domain = || -> Result<(), String> {
            let internal_settings = RpcToRust::internal_settings(req.internal_settings.unwrap())?;

            let buckets = RpcToRust::bucket_settings(req.buckets.unwrap())?;

            let origin = RpcToRust::origin_settings(req.origin_settings, None)?;

            let cool = Arc::new(DomainContext {
                domain: req.domain.clone(),
                origin: origin.clone(),
                api_engine_settings: RpcToRust::api_engine(req.api_engine_settings.unwrap(), None)?,
                caching_settings: RpcToRust::cache_settings(req.cache_settings.unwrap(), None)?,
                internal_settings: internal_settings.clone(),
                bot_settings: RpcToRust::bots(req.bot_settings.unwrap())?,
                human_engine_settings: RpcToRust::human_engine(
                    req.human_engine_settings.unwrap(),
                    None,
                )?,
                buckets: buckets.clone(),
                private_bucket: Arc::new(PrivateBucket::new().setup_defaults(&internal_settings)),
                analytic: Arc::new(Analytic::new()),
                rules: RpcToRust::page_rules(req.page_rules.unwrap(), None, buckets, origin)?,
                analytics_by_example: AnalyticsByExampleDomain {
                    started: Counter::new(),
                    examples: DashMap::new(),
                },
                app_settings: RpcToRust::app_settings(req.app_origin_settings, None)?,
            });

            DOMAINS_DB.insert(req.domain.clone(), cool);

            Ok(())
        };

        if let Err(e) = insert_domain() {
            return fail(&e);
        }

        success()
    }

    async fn delete_domain(
        &self,
        request: Request<DeleteDomainSchema>,
    ) -> Result<Response<QueryResponse>, Status> {
        GA.grpc.delete.inc();

        let req = request.into_inner();
        debug!("request domain: {}", req.domain);
        match DOMAINS_DB.remove(&req.domain) {
            Some(_) => success(),
            None => fail("Domain was attempted to be deleted, but it didn't exist"),
        }
    }

    async fn clear_cache(
        &self,
        request: Request<ClearCacheMessage>,
    ) -> Result<Response<QueryResponse>, Status> {
        GA.grpc.clear_cache.inc();

        // get the inner request
        let req = request.into_inner();

        debug!("Clear cache for {}", req.domain);

        // get the domain
        let domain_response = DOMAINS_DB.get(&req.domain);

        // make sure domain exists
        if let Some(domain) = domain_response {
            if let Some(everything) = req.everything {
                if everything {
                    // clear all the cache on this zone
                    domain.caching_settings.bucket.map.clear();

                    return success();
                }
            }

            if req.paths.is_empty() {
                return fail("No paths but all not selected!");
            };

            // it should be a specific path here
            let paths = req.paths;

            return if let Some(match_type) = req.r#type {
                // begin to iterate over the zone

                // the cached entries to end up deleting
                let mut to_delete = Vec::new();

                for v in domain.caching_settings.bucket.map.iter() {
                    // match the match type
                    match match_type {
                        0 => {
                            // exact path
                            // you might be asking me edward - this is really stupid
                            // you're right -- but I want to have complete consistency and
                            // performance issues are not a concern here
                            for path in paths.iter() {
                                if v.key() == path {
                                    to_delete.push(v.key().clone());

                                    break;
                                }
                            }
                        }
                        1 => {
                            // contains
                            for path in paths.iter() {
                                if v.key().contains(path) {
                                    to_delete.push(v.key().clone())
                                }
                            }
                        }
                        2 => {
                            // starts with
                            for path in paths.iter() {
                                if v.key().starts_with(path) {
                                    to_delete.push(v.key().clone());
                                }
                            }
                        }
                        3 => {
                            // ends with
                            for path in paths.iter() {
                                if v.key().ends_with(path) {
                                    to_delete.push(v.key().clone());
                                }
                            }
                        }
                        _ => return fail("Match type for clear cache path unimplemented"),
                    }
                }

                // now remove the actual references that we've dropped the iter read lock
                for i in to_delete {
                    if let Some(t) = domain.caching_settings.bucket.map.remove(&i) {
                        // delete it from Sled, too
                        drop(t.1);
                    }
                }

                success()
            } else {
                fail("Path existed, but match type did not")
            };
        } else {
            return fail("Domain not found");
        }
    }

    async fn challenge(&self, request: Request<Token>) -> Result<Response<QueryResponse>, Status> {
        GA.grpc.challenge.inc();

        let req = request.into_inner();

        debug!(
            "CHALLENGE INSERT: token: {}, message: {}",
            req.token.clone(),
            req.message.clone()
        );

        CHALLENGE_RESPONSES.insert(
            req.token.clone(),
            Arc::new(ChallengeResponse {
                token: req.token.to_string(),
                response: req.message.to_string(),
                epoch: 0,
            }),
        );

        success()
    }

    async fn challenge_removal(
        &self,
        request: Request<Token>,
    ) -> Result<Response<QueryResponse>, Status> {
        GA.grpc.challenge_removal.inc();

        let req = request.into_inner();

        debug!(
            "CHALLENGE DELETE: token: {}, message: {}",
            req.token.clone(),
            req.message.clone()
        );

        CHALLENGE_RESPONSES.remove(&req.token);

        success()
    }

    async fn challenge_completed(
        &self,
        request: Request<Cert>,
    ) -> Result<Response<QueryResponse>, Status> {
        GA.grpc.challenge_completed.inc();

        let req = request.into_inner();

        debug!("CHALLENGE COMPLETE: {}", req.domain.clone());

        match RpcToRust::cert(req) {
            Ok(_) => success(),
            Err(e) => fail(&e.to_string()),
        }
    }

    async fn refresh_challenge(
        &self,
        request: Request<SmartChallengeScript>,
    ) -> Result<Response<QueryResponse>, Status> {
        GA.grpc.challenge_shuffle.inc();

        let req = request.into_inner();

        use std::borrow::BorrowMut;
        use std::ops::DerefMut;

        debug!(
            "shuffling challenge: {}, {}, {}",
            req.bg_challenge_script.len(),
            req.smart_challenge_script.len(),
            req.key.len()
        );
        let mut bg = BACKGROUND_CHALLENGE.write().unwrap();
        let mut smart = SMART_CHALLENGE.write().unwrap();
        let mut keys = CHALLENGE_KEYS.write().unwrap();

        let _ = mem::replace(bg.deref_mut(), req.bg_challenge_script);
        let _ = mem::replace(smart.deref_mut(), req.smart_challenge_script);
        let _ = mem::replace(
            keys.deref_mut(),
            Aes128Gcm::new(GenericArray::from_slice(&req.key)),
        );

        return success();
    }
}
