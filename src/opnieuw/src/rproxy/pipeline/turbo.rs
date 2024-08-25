use crate::handler::pipeline::human_engine::smart_challenge::CookieTester;
use crate::models::egress_wrapper::EgressWrapper;
use crate::models::request_context::RequestContext;
use crate::rproxy::outbound_wrapper::HeartInsertable;
use hyper::body::Incoming;
use hyper::{HeaderMap, Response};

impl RequestContext {
    pub fn get_turbo(
        &self,
        response: &Response<Incoming>,
        headers: &mut HeaderMap,
    ) -> HeartInsertable {
        // if it's disabled, we can't activate it
        if !self.domain.human_engine_settings.turbo_mode_enabled {
            return HeartInsertable::No;
        }

        // now, let's check if the cookie already exists
        match self.token_tester_internal() {
            CookieTester::Good => return HeartInsertable::No,
            _ => {}
        };

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
