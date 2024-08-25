use crate::models::request_context::PipelineData;
use crate::HttpResponse;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::Response;

pub enum PipelineResponse {
    Ok(Option<Vec<PipelineData>>), // nothing needs to happen
    SkipPipeline(Vec<Pipelines>, Option<Vec<PipelineData>>), // skip specific pipelines
    // it doesn't make sense for the following to have PipelineData
    StopProcessing(HttpResponse), // return the response object now
    Error(String),                // error processing, return an error page
}

#[derive(Debug)] // so we can format! it if it errors
#[derive(PartialEq, Eq)] // need this flag so we can check for equality
pub enum Pipelines {
    // details which pipeline process to be skipped
    RequestInspection,
    ApiEngine,
    VerifiedBots,
    HumanEngine,
    Caching,
    Rules,
    All,
}
