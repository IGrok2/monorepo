use crate::templates::global_ratelimit::global_ratelimit_template;
use crate::{HttpResponse, GA};
use sentry::Level;

// an internal_error is an error on our side, not the clients
pub fn internal_error(error: &str) -> HttpResponse {
    // TODO: some logging should be done here too, error here is included for that
    GA.template.internal_error.inc();
    sentry::capture_message(error, Level::Error);
    return global_ratelimit_template(); // TODO internal error template
}
