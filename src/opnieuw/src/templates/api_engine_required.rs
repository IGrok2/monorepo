use crate::models::request_context::RequestContext;
use crate::utils::resp::resp;
use crate::{HttpResponse, GA};
use hyper::StatusCode;

impl RequestContext {
    pub fn api_engine_required(&self) -> HttpResponse {
        self.domain.analytic.api_engine_blocked.inc();
        GA.template.api_engine_required.inc();
        resp(
            "WebSocket connections must be allowed through the API Engine",
            Some(StatusCode::FORBIDDEN),
            false,
        )
    }
}
