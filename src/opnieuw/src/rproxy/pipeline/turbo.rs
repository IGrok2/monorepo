use crate::{
    handler::pipeline::human_engine::smart_challenge::CookieTester,
    models::{
        egress_wrapper::EgressWrapper,
        request_context::RequestContext,
    },
    rproxy::outbound_wrapper::HeartInsertable,
};
use hyper::{
    body::Incoming,
    HeaderMap,
    Response,
};

impl RequestContext {
    pub fn get_turbo(
        &self,
        response: &Response<Incoming>,
        headers: &mut HeaderMap,
    ) -> HeartInsertable {
        // if it's disabled, we can't activate it
        // also, if the cookie already exists, don't have them redo it
        if !self.domain.human_engine_settings.turbo_mode_enabled
            || matches!(self.token_tester_internal(), CookieTester::Good)
        {
            return HeartInsertable::No;
        }

        // now, if it is enabled, let's make sure it's the right content type
        if let Some(t) = response.headers().get("content-type") {
            if t != "text/html" {
                // invalid content type
                return HeartInsertable::No;
            }
        } else {
            // there's no content type, no assumptions
            return HeartInsertable::No;
        }

        // now, everything's good, so let's append to the content length header if it exists
        if let Some(t) = response.headers().get("content-length") {
            if let Ok(t) = t.to_str() {
                if let Ok(t) = t.parse::<u64>() {
                    headers.insert("content-length", (t + 35).to_string().parse().unwrap());
                }
            }
        }

        HeartInsertable::Yes
    }
}
