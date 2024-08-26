use crate::{
    debug,
    HttpResponse,
    GA,
};
use hyper::StatusCode;

use crate::utils::resp::resp;

// an internal_error is an error on our side, not the clients
pub fn invalid(_reason: &str) -> HttpResponse {
    //println!("{}", error);
    // TODO: some logging should be done here too, error here is included for that
    // TODO: request IDs
    GA.template.invalid.inc();

    debug!("INVALID REASON: {}", _reason);

    resp(r##"Invalid request"##, Some(StatusCode::BAD_REQUEST), true)
}
