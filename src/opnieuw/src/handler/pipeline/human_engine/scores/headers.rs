use crate::{
    models::request_context::RequestContext,
    GA,
};
use hyper::header::HeaderValue;

// TODO: have the user control this!
// params are the following:
//      1. First string: the name of the header
//      2. HeaderValue: the header as a HeaderValue
//      2. option'd string: None if it just needs to be there (or lack therof), Some if it has to be a certain value
//          Value it has to be equal to: Some(HeaderValue::from_static("Accept"))
//      3. True if the points are added if the header is there and meets option, false if the inverse
//      4. How much this should add or subtract
lazy_static! ( // because of HeaderValue we're gonna have to lazy_static this one
    static ref THREAT_SCORE: Vec<(String, Option<HeaderValue>, bool, u32)> = Vec::from([
        ("Accept-Encoding".to_string(), None, false, 20),
        ("Accept".to_string(), None, false, 50),
        ("X-Forwarded-For".to_string(), None, true, 40),
        ("X-Real-IP".to_string(), None, true, 40),
        ("X-Proxy-IP".to_string(), None, true, 50),
        ("via".to_string(), None, true, 50)
    ]);
);

impl RequestContext {
    pub fn check_headers(&self) -> u32 {
        // return the threat score from these headers
        // Basic header checking
        let mut score: u32 = 0;

        for rule in THREAT_SCORE.iter() {
            if rule.2 {
                // if we're adding points if the header is there, or equals a certain value
                match &rule.1 {
                    Some(t) => {
                        if self.req.headers.get(rule.0.clone()) == Some(t) {
                            score += rule.3
                        }
                    }
                    None => {
                        if self.req.headers.get(rule.0.clone()).is_some() {
                            score += rule.3
                        }
                    }
                }
            } else {
                // if we're adding points if the header is not there, or doesn't equal a certain value
                match &rule.1 {
                    Some(t) => {
                        // we want to make sure that requests don't equal a certain value
                        if self.req.headers.get(rule.0.clone()) != Some(t) {
                            score += rule.3
                        }
                    }
                    None => {
                        if self.req.headers.get(rule.0.clone()).is_none() {
                            score += rule.3
                        }
                    }
                }
            }
            if score > 100 {
                GA.handler.he.headers.inc();

                return score;
            }
        }

        if self.req.headers.len() <= 5 {
            // real users should be sending much more data
            score += 50
        }

        if score != 0 {
            GA.handler.he.headers.inc();
        }

        score
    }
}
