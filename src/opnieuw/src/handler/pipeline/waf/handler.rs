use crate::models::{
    pipeline_response::PipelineResponse,
    request_context::{
        PipelineData,
        RequestContext,
    },
};

impl RequestContext {
    pub fn waf(&self, _data: &[PipelineData]) -> PipelineResponse {
        todo!()
    }
}
