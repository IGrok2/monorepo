use crate::models::request_context::RequestContext;
use crate::{HttpResponse, GA};
use hyper::StatusCode;

use crate::utils::resp::resp;

impl RequestContext {
    pub fn too_many_requests(&self) -> HttpResponse {
        self.domain.analytic.ratelimited.inc();
        GA.template.too_many_requests.inc();
        resp(
            "Too many requests for this IP",
            Some(StatusCode::TOO_MANY_REQUESTS),
            true,
        )
    }
}
