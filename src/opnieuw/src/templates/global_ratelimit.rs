use crate::{
    HttpResponse,
    GA,
};
use hyper::StatusCode;

use crate::utils::resp::resp;

pub fn global_ratelimit_template() -> HttpResponse {
    GA.template.global_ratelimit.inc();
    resp(
        "IP is globally ratelimited",
        Some(StatusCode::TOO_MANY_REQUESTS),
        true,
    )
}
