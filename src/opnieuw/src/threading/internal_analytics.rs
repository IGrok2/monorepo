// stores up to 100 requests every 10 seconds, each request has a 10% chance of being chosen before it cuts out
// covers things that pass the challenges, and things that do not

use crate::{
    models::request_context::RequestContext,
    utils::counter::Counter,
};
use std::{
    ops::Deref,
    sync::RwLock,
};

lazy_static! {
    static ref INTERNAL_ANALYTICS: RwLock<Vec<RandomAnalytic>> = RwLock::new(Vec::new());
    static ref COUNT: Counter = Counter::new();
}

#[derive(Clone)]
pub struct RandomAnalytic {
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub method: String,
    pub query_string: String,
    pub country: String,
    pub asn: String,
}

// we're taking such raw types because we want to get the data before we modify it
impl RequestContext {
    pub fn collect_analytics(&self) {
        let req = self.req.clone();

        if COUNT.inc().get() > 10 {
            let mut head = vec![];
            let mut index = 0;
            for i in req.headers.iter() {
                index += 1;
                if index > 25 {
                    break;
                }
                head.push((i.0.to_string(), i.1.to_str().unwrap_or("NA").to_string()))
            }

            let ip_data = self.get_ipdata();

            INTERNAL_ANALYTICS.write().unwrap().push(RandomAnalytic {
                path: req.uri.path().to_string(),
                headers: head,
                method: req.method.to_string(),
                query_string: req.uri.query().unwrap_or("").to_string(),
                country: ip_data.0.to_string(),
                asn: ip_data.2.to_string(),
            })
        }
    }
}

/*
pub fn get_and_clear_analytics() -> Vec<RandomAnalytic> {
    let resp = INTERNAL_ANALYTICS.read().unwrap().deref().clone().to_vec();

    INTERNAL_ANALYTICS.write().unwrap().clear();
    COUNT.get_and_reset();

    resp
}

 */
