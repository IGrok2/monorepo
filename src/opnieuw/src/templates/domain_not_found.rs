use crate::utils::resp::resp;
use crate::{HttpResponse, GA};
use hyper::StatusCode;

pub fn domain_not_found() -> HttpResponse {
    GA.template.domain_not_found.inc();
    resp(
        "Couldn't locate that domain in our database",
        Some(StatusCode::NOT_FOUND),
        true,
    )
}
