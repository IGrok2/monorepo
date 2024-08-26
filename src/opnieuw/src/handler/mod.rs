use crate::models::{
    pipeline_response::PipelineResponse,
    request_context::{
        PipelineData,
        RequestContext,
    },
};

pub mod index;
// pub mod error;
pub mod cgi;
pub mod global_ratelimit;
pub mod models;
pub mod pipeline;
mod well_known;

pub type PipelineFunction = fn(&RequestContext, &Vec<PipelineData>) -> PipelineResponse;
