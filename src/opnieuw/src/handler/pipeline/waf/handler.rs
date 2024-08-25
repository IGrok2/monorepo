use crate::models::pipeline_response::PipelineResponse;
use crate::models::request_context::{PipelineData, RequestContext};

impl RequestContext {
    pub fn waf(&self, _data: &Vec<PipelineData>) -> PipelineResponse {
        todo!()
    }
}
