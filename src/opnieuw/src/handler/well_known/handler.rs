use crate::{
    models::pipeline_response::PipelineResponse,
    HttpResponse,
    CHALLENGE_RESPONSES,
    GA,
};
use http_body_util::{
    combinators::BoxBody,
    Full,
};
use hyper::{
    body::Bytes,
    Response,
    StatusCode,
};

use crate::utils::resp::resp;

pub fn well_known_handler(token: String) -> PipelineResponse {
    match CHALLENGE_RESPONSES.get(&token) {
        Some(t) => {
            // increment counter showing that the token was found
            GA.cgi.well_known_token_found.inc();

            // return back the response
            PipelineResponse::StopProcessing(resp(t.response.as_str(), Some(StatusCode::OK), true))
        }
        None => {
            // increment counter reflecting that we did not find this record
            GA.cgi.well_known_token_not_found.inc();

            // return back the response
            PipelineResponse::Ok(None)
        }
    }
}
