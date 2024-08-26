use crate::{
    models::request_context::RequestContext,
    HttpResponse,
    GA,
};
use hyper::StatusCode;

use crate::utils::resp::resp;

impl RequestContext {
    pub fn serve_waf(&self) -> HttpResponse {
        self.domain.analytic.waf_block.inc();
        GA.template.waf.inc();
        resp(
            "Connection blocked by WAF",
            Some(StatusCode::UNAUTHORIZED),
            true,
        )
    }
}
