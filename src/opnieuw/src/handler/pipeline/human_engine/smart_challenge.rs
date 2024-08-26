use crate::buckets::models::PublicBucket;
use crate::handler::pipeline::human_engine::cookie::ChallengeCookie;
use crate::models::pipeline_response::PipelineResponse;
use crate::models::request_context::RequestContext;
use crate::tls::models::TlsFingerprint;
use crate::GA;
use bytes::Bytes;

// for tokens

#[derive(PartialEq, Eq)]
pub enum CookieTester {
    Bad,
    NoCookie,
    Good,
}

impl RequestContext {
    pub fn cookie_tester_internal(&self) -> CookieTester {
        // TODO: change to tokens
        // TODO: this won't work
        if let Some(cookie) = self.get_cookie("__pw_loves_you") {
            if let Some(t) = ChallengeCookie::get_cookie(&cookie) {
                if t.ip == self.ip.ip && t.website == self.domain.domain && t.ua == self.user_agent
                {
                    if let Some(v) = self.connection_context.fingerprint {
                        if v != t.tls_fingerprint {
                            GA.cookie.cookie_misused.inc();

                            return CookieTester::Bad;
                        }
                    }

                    GA.cookie.cookie_worked.inc();

                    self.domain.analytic.cookie_verified.inc();

                    use std::ops::DerefMut;
                    if let Some(ref mut t) = self.by_example.borrow_mut().deref_mut() {
                        t.human_engine.cookie_validated = true;
                    }

                    // cookie worked, let's end it here
                    return CookieTester::Good;
                } else {
                    GA.cookie.cookie_misused.inc();

                    return CookieTester::Bad;
                }
            } else {
                GA.cookie.cookie_failed.inc();

                return CookieTester::Bad;
            }
        }

        CookieTester::NoCookie
    }

    // token tester
    pub fn token_tester_internal(&self) -> CookieTester {
        let read = self.ip.protected.read().unwrap();

        // now that we have the read, let's check the tokens

        for token in read.tokens.iter() {
            if token.0.user_agent == self.user_agent {
                if let Some(t) = self.connection_context.fingerprint {
                    match token.0.fingerprint {
                        TlsFingerprint::Chrome => match t {
                            TlsFingerprint::Chrome => {}
                            _ => return CookieTester::Bad,
                        },
                        TlsFingerprint::Safari => match t {
                            TlsFingerprint::Safari => {}
                            _ => return CookieTester::Bad,
                        },
                        TlsFingerprint::Firefox => match t {
                            TlsFingerprint::Firefox => {}
                            _ => return CookieTester::Bad,
                        },
                        TlsFingerprint::Unknown => match t {
                            TlsFingerprint::Unknown => {}
                            _ => return CookieTester::Bad,
                        },
                    }

                    // token is good
                    if token.0.points.inc_by(1).get() < 5000 {
                        // 5000 requests per token
                        return CookieTester::Good;
                    }
                    return CookieTester::Bad;
                }
            }
        }

        CookieTester::NoCookie
    }

    // expected to return either Ok or StopProcessing depending on if the challenge is to be served
    pub fn smart_challenge_manager(&self) -> PipelineResponse {
        match self.token_tester_internal() {
            CookieTester::Good => PipelineResponse::Ok(None),
            CookieTester::Bad | CookieTester::NoCookie => {
                PipelineResponse::StopProcessing(self.smart_challenge())
            }
        }
    }
}

pub fn get_background_challenge() -> Bytes {
    Bytes::from(r##"<script src="/__pw/bg-js"></script>"##)
}
