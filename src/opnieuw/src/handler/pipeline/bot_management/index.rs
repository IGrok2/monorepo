use crate::{
    handler::pipeline::bot_management::models::IsBotResponse,
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
};

/* 3: Verified bots
- Checks if user would like to allow bots we consider verified
    - Checks if requester is bot or not
        - Uses default ratelimiting bucket (`domain_bots`) for bots per domain for ALL bots to make sure
            bots don't flood a zone
 */

impl RequestContext {
    pub fn bot_management(&self, _data: &[PipelineData]) -> PipelineResponse {
        if !self.domain.bot_settings.enabled || self.domain.bot_settings.bots_allowed.is_empty() {
            return PipelineResponse::Ok(None);
        }

        match self.is_bot() {
            IsBotResponse::Bot => {
                use std::ops::DerefMut;
                if let Some(ref mut t) = self.by_example.borrow_mut().deref_mut() {
                    t.bot_management.allowed = true;
                }

                PipelineResponse::SkipPipeline(Vec::from([Pipelines::HumanEngine]), None)
            }
            IsBotResponse::NotBot => PipelineResponse::Ok(None),
            IsBotResponse::Ratelimited => {
                PipelineResponse::StopProcessing(self.too_many_requests()) // already been reported by the bucket
            }
        }
    }
}
