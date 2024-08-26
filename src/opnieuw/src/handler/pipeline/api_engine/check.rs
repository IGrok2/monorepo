// check that a path is covered by the API engine and whether it accepts WS or not

use std::rc::Rc;

use crate::{
    buckets::models::PublicBucket,
    handler::pipeline::api_engine::models::{
        Method as ApiMethod,
        Method,
        Rule,
        TriggerType,
        WebMethods,
    },
    models::request_context::RequestContext,
};

// sends back if a WS is allowed here, along with an optional ratelimit bucket to use for that path
// actually, this shouldn't be here, because the api engine will already block WS connections that shouldn't be there
// so, instead, we will just return an optional RL bucket
pub fn ws_check(_context: &RequestContext) -> Option<Rc<PublicBucket>> {
    todo!()
}

pub fn is_api_engine(path: &str, expected_path: &str) -> bool {
    if path.contains(expected_path) {
        // this won't work well with *
        return true;
    }
    false
}

impl RequestContext {
    pub fn matches_method(&self, wanted_method: &Vec<ApiMethod>) -> bool {
        for i in wanted_method {
            match i {
                ApiMethod::Web(t) => {
                    let method = self.req.method.clone();

                    for i in t {
                        match i {
                            WebMethods::Get => {
                                if method.as_str() == "GET" {
                                    // we don't want to return false if it doesn't match, because it could easily match another method
                                    return true;
                                }
                            }
                            WebMethods::Post => {
                                if method.as_str() == "POST" {
                                    return true;
                                }
                            }
                            WebMethods::Options => {
                                if method.as_str() == "OPTIONS" {
                                    return true;
                                }
                            }
                            WebMethods::Delete => {
                                if method.as_str() == "DELETE" {
                                    return true;
                                }
                            }
                            WebMethods::Head => {
                                if method.as_str() == "HEAD" {
                                    return true;
                                }
                            }
                            WebMethods::Put => {
                                if method.as_str() == "PUT" {
                                    return true;
                                }
                            }
                            WebMethods::Connect => {
                                if method.as_str() == "CONNECT" {
                                    return true;
                                }
                            }
                            WebMethods::Trace => {
                                if method.as_str() == "TRACE" {
                                    return true;
                                }
                            }
                            WebMethods::Patch => {
                                if method.as_str() == "PATCH" {
                                    return true;
                                }
                            }
                        }
                    }
                }
                ApiMethod::Ws(_w) => {
                    // this is a websocket path
                    return self.is_ws(); // we will use the methods later, when we start to handle the actual message
                }
            }
        }
        false
    }
}

impl RequestContext {
    pub fn is_ws(&self) -> bool {
        if let Some(t) = self.req.headers.get("Upgrade") {
            if let Ok(t) = t.to_str() {
                return t == "websocket";
            }
        }
        false
    }
}

pub fn matches(ctx: &RequestContext, rule: &Rule) -> bool {
    let mut allowed = false;

    for i in rule.methods_allowed.iter() {
        match i {
            Method::Ws(_t) => {
                if ctx.is_ws() {
                    allowed = true;
                    break;
                }
            }
            Method::Web(t) => {
                for v in t.iter() {
                    match v {
                        WebMethods::Get => {
                            if ctx.req.method == "GET" {
                                allowed = true;
                            }
                        }
                        WebMethods::Post => {
                            if ctx.req.method == "POST" {
                                allowed = true;
                            }
                        }
                        WebMethods::Options => {
                            if ctx.req.method == "OPTIONS" {
                                allowed = true;
                            }
                        }
                        WebMethods::Delete => {
                            if ctx.req.method == "DELETE" {
                                allowed = true;
                            }
                        }
                        WebMethods::Head => {
                            if ctx.req.method == "HEAD" {
                                allowed = true;
                            }
                        }
                        WebMethods::Put => {
                            if ctx.req.method == "PUT" {
                                allowed = true;
                            }
                        }
                        WebMethods::Connect => {
                            if ctx.req.method == "CONNECT" {
                                allowed = true;
                            }
                        }
                        WebMethods::Trace => {
                            if ctx.req.method == "TRACE" {
                                allowed = true;
                            }
                        }
                        WebMethods::Patch => {
                            if ctx.req.method == "PATCH" {
                                allowed = true;
                            }
                        }
                    }
                }
            }
        }
    }

    if !allowed {
        return false;
    }

    let wanted: &Vec<String> = &rule.path;
    let real: &str = ctx.req.uri.path();

    for path_wanted in wanted {
        match rule.trigger_type {
            TriggerType::Exact => {
                if path_wanted == real {
                    return true;
                }
            }
            TriggerType::Contains => {
                if real.contains(path_wanted) {
                    return true;
                }
            }
            TriggerType::StartsWith => {
                if real.starts_with(path_wanted) {
                    return true;
                }
            }
            TriggerType::UseStar => { // TODO implement
            }
        };
    }

    false
}

// TODO: header check

// TODO: path check

// TODO: THESE SHOULD BE IN THE RULES PACKAGE ^^^
