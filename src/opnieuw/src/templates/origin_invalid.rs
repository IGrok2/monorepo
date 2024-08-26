use crate::{
    models::request_context::RequestContext,
    HttpResponse,
    GA,
};
use hyper::StatusCode;

use crate::utils::resp::resp;

impl RequestContext {
    pub fn origin_invalid(&self) -> HttpResponse {
        self.domain.analytic.origin_err_reqs.inc();
        GA.template.origin_invalid.inc();
        resp(
            "Origin setting is invalid!",
            Some(StatusCode::LOOP_DETECTED),
            true,
        )
    }
}
