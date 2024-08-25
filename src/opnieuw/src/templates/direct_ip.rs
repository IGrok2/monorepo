use crate::utils::resp::resp;
use crate::{HttpResponse, GA};
use hyper::StatusCode;

pub fn direct_ip_reject() -> HttpResponse {
    GA.template.direct_ip.inc();
    resp(
        "Please access domains through their actual domain, not the IP",
        Some(StatusCode::FORBIDDEN),
        false,
    )
}
