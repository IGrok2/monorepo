use crate::buckets::models::PublicBucket;
use crate::handler::models::ConnectionContext;
use crate::ip::models::TrafficType;

use crate::models::pipeline_response::PipelineResponse;
use crate::templates::global_ratelimit::global_ratelimit_template;

impl ConnectionContext {
    pub fn global_ratelimit(&self) -> PipelineResponse {
        match self.ip.allow(TrafficType::NewRequest) {
            // if we include Some for create, this func will never return None
            true => PipelineResponse::Ok(None),
            false => PipelineResponse::StopProcessing(global_ratelimit_template()),
        }
    }
}
