use crate::handler::pipeline::rules::models::{Action, Monopoly, Trustbusting};
use crate::models::domain_context::OriginType;
use crate::models::pipeline_response::{PipelineResponse, Pipelines};
use crate::models::request_context::{PipelineData, RequestContext};
use crate::utils::redirect::perform_redirect;
use crate::{debug, DOMAINS_DB, GA};
use std::sync::Arc;

pub fn do_action(ctx: &RequestContext, action: &Action) -> Option<PipelineResponse> {
    GA.handler.pr.did_action.inc();

    debug!("Doing action: {:?}", action);

    let mut skip_data = Vec::new();

    return match action {
        Action::Monopoly(t) => match t {
            Monopoly::Block => {
                GA.handler.pr.block.inc();

                Some(PipelineResponse::StopProcessing(ctx.blocked()))
            }
        },
        Action::Trustbusting(t) => {
            let mut data: Vec<PipelineData> = vec![];

            for i in t.iter() {
                match i {
                    Trustbusting::SmartChallenge => {
                        GA.handler.pr.challenge.inc();

                        if let PipelineResponse::StopProcessing(resp) =
                            ctx.smart_challenge_manager()
                        {
                            return Some(PipelineResponse::StopProcessing(resp));
                        }
                    } // this should return a bool if it's good, and return a challenge if not
                    Trustbusting::CaptchaChallenge => {} // TODO remove
                    Trustbusting::RatelimitBucket(bucket) => {
                        debug!("Ratelimit bucket action is being processed");

                        GA.handler.pr.buckets.inc();

                        if !bucket.check_ip(&ctx.ip) {
                            return Some(PipelineResponse::StopProcessing(ctx.too_many_requests()));
                        }
                    }
                    Trustbusting::Cache(level, duration) => {
                        GA.handler.pr.cache_data.inc();

                        data.push(PipelineData {
                            cache_level: Some((
                                *level,
                                duration.unwrap_or(ctx.domain.caching_settings.default_cache_ttl),
                            )),
                            bucket: None,
                            ws_methods: None,
                            backend: None,
                        })
                    }
                    Trustbusting::Redirect(to) => {
                        GA.handler.pr.redirect.inc();

                        return Some(PipelineResponse::StopProcessing(perform_redirect(&to)));
                    }
                    Trustbusting::UseBackend(backend) => {
                        GA.handler.pr.use_backend.inc();

                        if let Some(t) = ctx.domain.origin.settings.get(backend) {
                            data.push(PipelineData {
                                cache_level: None,
                                bucket: None,
                                ws_methods: None,
                                backend: Some(t.value().clone()),
                            })
                        } else {
                            GA.handler.pr.backend_not_found.inc();

                            return Some(PipelineResponse::StopProcessing(ctx.origin_invalid()));
                        }
                    }
                    Trustbusting::Rewrite(proto) => {
                        return Some(PipelineResponse::StopProcessing(perform_redirect(
                            &(proto.clone() + "://" + &ctx.full_host + ctx.req.uri.path()),
                        )));
                    }
                    Trustbusting::SkipHumanEngine => skip_data.push(Pipelines::HumanEngine),
                    Trustbusting::UseApp(app) => {
                        GA.handler.pr.use_backend.inc();

                        data.push(PipelineData {
                            cache_level: None,
                            bucket: None,
                            ws_methods: None,
                            backend: Some(Arc::new(OriginType::App(app.clone()))),
                        })
                    }
                }
            }

            let data_option = match data.len() {
                0 => None,
                _ => Some(data),
            };

            if !skip_data.is_empty() {
                return Some(PipelineResponse::SkipPipeline(skip_data, data_option));
            }

            if let Some(data) = data_option {
                return Some(PipelineResponse::Ok(Some(data)));
            }

            None
        }
    };
}
