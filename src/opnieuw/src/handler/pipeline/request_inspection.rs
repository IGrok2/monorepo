// Inspect the request -- for DDoS *and* WAF
// This file should utilize the "block" threading

use crate::{
    models::{
        pipeline_response::PipelineResponse,
        request_context::{
            PipelineData,
            RequestContext,
        },
    },
    templates::domain_not_found::domain_not_found,
    GA,
};

impl RequestContext {
    pub fn request_inspection(&self, _data: &[PipelineData]) -> PipelineResponse {
        /*
        THE FOLLOWING SHOULD NOT BE IN THE USERS' CONTROL
        Malicious traffic
         */

        // if the domain has been suspended
        if self.domain.internal_settings.domain_blocked {
            // then, we mark up this domain's threading saying it is invalid
            GA.handler.ri.domain_blocked.inc();
            self.domain.analytic.invalid_reqs.inc();
            // finally, we return a response saying this can't work
            return PipelineResponse::StopProcessing(domain_not_found());
        }

        // here's where we get to the juicy security parts ...

        // this check is to make sure the request isn't looping
        // we add CDN-loop on the egress request
        if self.req.headers.get("CDN-loop").is_some() {
            GA.handler.ri.cdn_loop.inc();
            self.domain.analytic.invalid_reqs.inc();
            // we can clone here because it's an Arc so it's just a reference
            return PipelineResponse::StopProcessing(self.origin_invalid());
        }

        // weird user agents / headers
        if self.user_agent.len() > 400
            || self.user_agent.is_empty()
            || self.req.headers.is_empty()
            || self.req.headers.len() > 1000
        {
            // certain checks all requests must pass
            GA.handler.ri.weird_user.inc();
            return PipelineResponse::StopProcessing(self.serve_waf());
        }

        if self.req.uri.path() == ".env" {
            // certain checks all requests must pass
            GA.handler.ri.waf.inc();
            return PipelineResponse::StopProcessing(self.serve_waf());
        }
        /*
                // headervalues generate errors if they don't follow standards. we convert later in the code, this is a possible security hole!
                for i in self.req.headers.iter() { // TODO: evaluate performance concerns
                    if i.1.to_str().is_err() {
                        g_analytic(Action::Handler(Handler::RequestInspection(RequestInspection::HeaderNotStr)));
                        self.domain.analytic.invalid_reqs.inc(); // TODO: we gotta make custom counter for this
                        return PipelineResponse::StopProcessing(resp("Connection blocked", Some(StatusCode::BAD_REQUEST), true))
                    }
                }
        */
        /*
        THE FOLLOWING CAN BE IN THE USERS CONTROL
         */

        /* no longer done here! TODO: move to threat score check instead
            // check basic headers to verify the integrity of this request
            if self.domain.general_security_settings.request_inspection && (self.req.head().headers.get("X-Forwarded-For").is_some() || self.req.head().headers.get("X-Real-Ip").is_some() || self.req.head().headers.get("X-Proxy-Ip").is_some() || self.req.head().headers.get("via").is_some()) {
                self.domain.analytic.managed_blocked_reqs.inc();
                return PipelineResponse::StopProcessing(resp("Connection blocked", Some(StatusCode::UNAUTHORIZED), true))
            }
        */

        PipelineResponse::Ok(None)
    }
}
