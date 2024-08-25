use crate::models::request_context::RequestContext;
use crate::{HttpResponse, GA};
use hyper::StatusCode;

use crate::utils::resp::resp;

impl RequestContext {
    pub fn too_many_requests_for_backend(&self) -> HttpResponse {
        GA.template.rl_backend.inc();
        self.domain.analytic.internally_ratelimited.inc();
        // here, we're going to let the handlers manage the internal ratelimited counter and not manage it from this template
        resp(
            "Too many requests for this domain",
            Some(StatusCode::TOO_MANY_REQUESTS),
            true,
        )
    }
}
