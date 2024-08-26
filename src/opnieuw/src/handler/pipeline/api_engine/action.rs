use crate::{
    handler::pipeline::api_engine::models::{
        Action,
        Method,
        Rule,
        Setting,
    },
    models::{
        pipeline_response::{
            PipelineResponse,
            Pipelines,
        },
        request_context::{
            PipelineData,
            RequestContext,
        },
    },
    GA,
};

// TODO: inspect
impl RequestContext {
    pub fn do_action(&self, action: &Rule, setting: &Setting) -> PipelineResponse {
        // ws_methods should only be fed if it's a websocket connection
        let mut data = vec![];

        for i in action.action.iter() {
            match i {
                Action::Ratelimit(t) => {
                    GA.handler.ae.ratelimit.inc();

                    if !t.check_ip(&self.ip) {
                        // bucket will take care of the ratelimiting
                        GA.handler.ae.ratelimited.inc();

                        return PipelineResponse::StopProcessing(self.too_many_requests());
                    }

                    if self.is_ws() {
                        // matched and is websocket
                        let mut methods = None;
                        for i in action.methods_allowed.iter() {
                            if let Method::Ws(i) = i {
                                methods = Some(i)
                            }
                        }
                        data.push(PipelineData {
                            cache_level: None,
                            bucket: Some(t.clone()),
                            ws_methods: Some(methods.unwrap().clone()),
                            backend: None,
                        }) // unwrap to ensure it exists
                    }
                }
                Action::Cache(t, duration) => {
                    GA.handler.ae.cache.inc();

                    data.push(PipelineData {
                        cache_level: Some((
                            *t,
                            duration.unwrap_or(self.domain.caching_settings.default_cache_ttl),
                        )),
                        bucket: None,
                        ws_methods: None,
                        backend: None,
                    })
                }
            }
        }

        if setting.open_api {
            if data.is_empty() {
                PipelineResponse::SkipPipeline(
                    Vec::from([Pipelines::VerifiedBots, Pipelines::HumanEngine]),
                    None,
                )
            } else {
                PipelineResponse::SkipPipeline(
                    Vec::from([Pipelines::VerifiedBots, Pipelines::HumanEngine]),
                    Some(data),
                )
            }
        } else if data.is_empty() {
            PipelineResponse::SkipPipeline(Vec::from([Pipelines::VerifiedBots]), None)
        } else {
            PipelineResponse::SkipPipeline(Vec::from([Pipelines::VerifiedBots]), Some(data))
        }
        // default action
    }
}
