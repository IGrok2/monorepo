use crate::models::domain_context::DomainContext;
use crate::models::request_context::RequestContext;
use crate::{HttpResponse, GA};
use hyper::StatusCode;

use crate::utils::resp::resp;

impl DomainContext {
    pub fn origin_down(&self) -> HttpResponse {
        GA.template.origin_down.inc();
        self.analytic.origin_err_reqs.inc();
        resp(
            r##"
        Origin down
    "##,
            Some(StatusCode::INTERNAL_SERVER_ERROR),
            true,
        )
    }
}
