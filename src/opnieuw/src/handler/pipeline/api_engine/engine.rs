use crate::{
    handler::pipeline::{
        api_engine::{
            check::matches,
            models,
            models::{
                Action,
                WhitelistFactors,
            },
        },
        human_engine::smart_challenge::CookieTester,
    },
    GA,
};
use hyper::Method;

use crate::models::{
    pipeline_response::{
        PipelineResponse,
        Pipelines,
    },
    request_context::{
        PipelineData,
        RequestContext,
    },
};

// This is the main handler of the data

/* 2: API engine
   - NOTE: ratelimits should be MOSTLY global per ZONE.
   - Users should have a maximum of 3 ratelimiting BUCKETS (these are leaky buckets) that can be applied to paths
       1. checks if this path is handled by the api api_engine
           - if not, ends the pipeline there
       2. checks if the method is allowed (also is it a websocket and is that allowed & add this to req context)
           - if not, blocks the request with a nice page (or optionally, a page the user selects)
       3. checks if this request should be allowed by ratelimit, acquire bucket for this path and it to req context
           - if not, throws over a 429 with a page
       4. checks if this request should be verified (to check if it's a JWT verified user)
           - if not, return 401 unauthorized with nice page
       5. checks if it's a websocket connection request
           - if so,
               1. checks the WAF (if the user has it on, escapes JS characters for example)
               2. checks rules (does this websocket path allow binary, ping or text?)
               3. sends it to backend
               4. handles all proceeding communication, continuing to check if they're hitting the ratelimit for that bucket

       6. checks if this request needs to be micro-cached (quick cached or long cache)
           - this only works for GET requests
           - if so,
               1. has the cache utility pull from Sled
*/
// MASSIVE TODO: MAKE SURE TO ADJUST IF THE STREAMS ARE ALLOWED!
impl RequestContext {
    pub fn api_engine(&self, _data: &[PipelineData]) -> PipelineResponse {
        let ws = self.is_ws();

        if (!self.domain.api_engine_settings.enabled
            || self.domain.api_engine_settings.rules.is_empty())
            && ws
        {
            // websocket connection without the api engine being on? outta here!
            GA.handler.ae.ws_no_engine.inc();

            use std::ops::DerefMut;
            if let Some(ref mut t) = self.by_example.borrow_mut().deref_mut() {
                t.api_engine.done = true;
                t.api_engine
                    .actions
                    .push("The API Engine must explicitly allow WebSocket connections".to_string())
            }

            return PipelineResponse::StopProcessing(self.api_engine_blocked()); // this is a websocket connection, we can't let this through
        }

        if !self.domain.api_engine_settings.enabled
            || self.domain.api_engine_settings.rules.is_empty()
        {
            // ok it's not websocket and this user has no settings, so we can let this through
            return PipelineResponse::Ok(None);
        }

        let mut setting = None;

        for i in self.domain.api_engine_settings.rules.iter() {
            if (self.full_host.clone() + self.req.uri.path()).starts_with(&i.host_path) {
                // TODO: make use of those * things here
                setting = Some(i)
            }
        }

        if setting.is_none() && !ws {
            // there's no available setting and it's not a websocket connection, let it fly!
            if self.domain.api_engine_settings.strict_mode && self.req.method != Method::GET {
                // it was a request other than a GET but wasn't for the api engine
                use std::ops::DerefMut;
                if let Some(ref mut t) = self.by_example.borrow_mut().deref_mut() {
                    t.api_engine.done = true;
                    t.api_engine.actions.push("Strict mode was enabled, but client wanted to stream data without a setting or rule".to_string())
                }

                return PipelineResponse::StopProcessing(self.api_engine_blocked());
                // TODO: specific api engine block analytics
            }

            return PipelineResponse::Ok(None);
        } else if setting.is_none() && ws {
            // websocket isn't allowed without a rule!
            GA.handler.ae.ws_no_engine.inc();

            use std::ops::DerefMut;
            if let Some(ref mut t) = self.by_example.borrow_mut().deref_mut() {
                t.api_engine.done = true;
                t.api_engine.actions.push(
                    "WebSockets cannot be used without an API engine rule allowing them"
                        .to_string(),
                )
            }

            return PipelineResponse::StopProcessing(self.api_engine_blocked());
        }

        let setting = setting.unwrap();

        for i in setting.whitelist_factors.iter() {
            // TODO: make sure this checks whether websocket connections are allowed, maybe an atomicbool
            GA.handler.ae.whitelist_factors_checked.inc();

            match i {
                // exact matches only
                WhitelistFactors::Ip(ip) => {
                    if ip == &self.ip.ip {
                        // TODO: analytic here
                        GA.handler.ae.whitelist_ip_hit.inc();

                        use std::ops::DerefMut;
                        if let Some(ref mut t) = self.by_example.borrow_mut().deref_mut() {
                            t.api_engine.done = true;
                            t.api_engine.setting = setting.host_path.clone();
                            t.api_engine
                                .actions
                                .push("IP was part of whitelist".to_string())
                        }

                        return PipelineResponse::Ok(None);
                    }
                }
                WhitelistFactors::Header(k, v) => {
                    if let Some(t) = self.req.headers.get(k) {
                        if &t.to_str().unwrap().to_string() == v {
                            GA.handler.ae.whitelist_header_hit.inc();

                            use std::ops::DerefMut;
                            if let Some(ref mut t) = self.by_example.borrow_mut().deref_mut() {
                                t.api_engine.done = true;
                                t.api_engine.setting = setting.host_path.clone();
                                t.api_engine
                                    .actions
                                    .push("Header key/value was part of whitelist".to_string())
                            }

                            return PipelineResponse::Ok(None);
                        }
                    }
                }
            }
        }

        for i in setting.rules.iter() {
            // the rules applicable to this paths that begin with the proper one
            if matches(self, i) {
                // check if its the right path
                i.hit.inc();

                // Query string check
                if !i.allow_query_string && self.req.uri.query().is_some() {
                    use std::ops::DerefMut;
                    if let Some(ref mut t) = self.by_example.borrow_mut().deref_mut() {
                        t.api_engine.done = true;
                        t.api_engine.setting = setting.host_path.clone();
                        t.api_engine.rule = i.id.clone();
                        t.api_engine.actions.push("Rule doesn't allow for query strings but the request had a query string".to_string())
                    }

                    // query string wasn't allowed yet it existed
                    return PipelineResponse::StopProcessing(self.api_engine_blocked());
                }

                // human engine validation
                if i.human_engine_validation {
                    // then, get the cookie status
                    if self.token_tester_internal() != CookieTester::Good {
                        use std::ops::DerefMut;
                        if let Some(ref mut t) = self.by_example.borrow_mut().deref_mut() {
                            t.api_engine.done = true;
                            t.api_engine.setting = setting.host_path.clone();
                            t.api_engine.rule = i.id.clone();
                            t.api_engine.actions.push(
                                "Rule wanted client to be verified, but the client was not"
                                    .to_string(),
                            )
                        }

                        return PipelineResponse::StopProcessing(self.api_engine_blocked());
                    }
                }

                // now we know this rule matches
                let mut websocket_allowed = None;
                if ws {
                    for v in i.methods_allowed.iter() {
                        match v {
                            models::Method::Ws(t) => websocket_allowed = Some(t.clone()), // we can allow this websocket connection
                            models::Method::Web(_) => {} // do nothing
                        }
                    }

                    if websocket_allowed.is_some() {
                        // it's a websocket and it's been allowed
                        let mut bucket = None;

                        match self.do_action(i, setting) {
                            PipelineResponse::Ok(_) => {}
                            PipelineResponse::SkipPipeline(pipelines, data) => {
                                return PipelineResponse::SkipPipeline(pipelines, data)
                            }
                            PipelineResponse::StopProcessing(resp) => {
                                return PipelineResponse::StopProcessing(resp)
                            }
                            PipelineResponse::Error(resp) => return PipelineResponse::Error(resp),
                        }

                        for i_ac in i.action.iter() {
                            // search for the bucket if there is one
                            match i_ac {
                                Action::Ratelimit(t) => {
                                    use std::ops::DerefMut;
                                    if let Some(ref mut t) =
                                        self.by_example.borrow_mut().deref_mut()
                                    {
                                        t.api_engine.done = true;
                                        t.api_engine.setting = setting.host_path.clone();
                                        t.api_engine.rule = i.id.clone();
                                        t.api_engine.actions.push(
                                            "Rule called for ratelimiting via bucket".to_string(),
                                        )
                                    }

                                    bucket = Some(t.clone())
                                }
                                Action::Cache(..) => {} // don't do anything here, because we can't
                            }
                        }

                        // this is a websocket connection, send it to the websocket handler
                        return if setting.open_api {
                            PipelineResponse::SkipPipeline(
                                Vec::from([Pipelines::All]),
                                Some(Vec::from([PipelineData {
                                    cache_level: None,
                                    bucket, // intentionally panic if there's nothing here
                                    ws_methods: Some(websocket_allowed.unwrap()), // intentionally panic if there's nothing here
                                    backend: None,
                                }])),
                            )
                        } else {
                            PipelineResponse::SkipPipeline(
                                Vec::from([
                                    Pipelines::Caching,
                                    Pipelines::VerifiedBots,
                                    Pipelines::Rules,
                                ]),
                                Some(Vec::from([PipelineData {
                                    cache_level: None,
                                    bucket, // intentionally panic if there's nothing here
                                    ws_methods: Some(websocket_allowed.unwrap()), // intentionally panic if there's nothing here
                                    backend: None,
                                }])),
                            )
                        };
                    }
                } else {
                    return self.do_action(i, setting);
                }
            }
        }

        GA.handler.ae.setting_found_no_rule.inc();

        // this path is part of the API engine, however, no rule matched (so block!)
        PipelineResponse::StopProcessing(self.api_engine_blocked())
    }
}
