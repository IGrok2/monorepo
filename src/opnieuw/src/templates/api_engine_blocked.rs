use crate::{
    models::request_context::RequestContext,
    utils::resp::resp,
    HttpResponse,
    GA,
};
use hyper::StatusCode;

impl RequestContext {
    pub fn api_engine_blocked(&self) -> HttpResponse {
        self.domain.analytic.api_engine_blocked.inc();
        GA.template.api_engine_blocked.inc();
        resp(
            "Connection blocked by API engine",
            Some(StatusCode::FORBIDDEN),
            false,
        )
    }
}
