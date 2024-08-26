use crate::buckets::models::PublicBucket;
use crate::handler::cgi::crypto::ParsedChallengeResponse;
use crate::handler::models::ConnectionContext;
use crate::handler::pipeline::human_engine::cookie::ChallengeCookie;
use crate::ip::models::{Token, NewTrafficType};
use crate::models::request_context::RequestContext;
use crate::templates::error::internal_error;
use crate::templates::invalid::invalid;
use crate::tls::models::TlsFingerprint;
use crate::utils::counter::Counter;
use crate::utils::cycle::Cycle;
use crate::utils::epoch::epoch;
use crate::utils::resp::resp;
use crate::{debug, HttpResponse, BACKGROUND_CHALLENGE, GA};
use aes_gcm::aead::{Aead, AeadMut};
use aes_gcm::KeyInit;
use futures_util::StreamExt;
use http_body_util::BodyExt;
use hyper::body::{Bytes, Frame, Incoming};
use hyper::{Method, StatusCode};
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;
use tokio::time::timeout;

impl RequestContext {
    pub async fn cgi(self, payload: Incoming) -> HttpResponse {
        GA.cgi.cgi_request.inc();

        let path = self.req.uri.path().to_string();

        match path.as_str() {
            "/__pw/submit-challenge" | "/__pw/bg" => {
                if self.req.method == Method::POST {
                    GA.cgi.submit_challenge_request.inc();

                    // TODO: cgi path analytics

                    let new_body = match timeout(Duration::from_secs(2), async move {
                        payload
                            .map_frame(|frame| {
                                if let Some(data) = frame.data_ref() {
                                    if data.len() > 2048 {
                                        return Frame::data(Bytes::new());
                                    }
                                }
                                frame
                            })
                            .collect()
                            .await
                    })
                    .await {
                        Ok(t) => match t {
                            Ok(v) => v,
                            Err(_e) => return invalid("too big"),
                        },
                        Err(_e) => {
                            GA.cgi.submit_challenge_timeout.inc();
                            return invalid("timeout");
                        }
                    };

                    let response =
                        match ParsedChallengeResponse::decrypt(new_body.to_bytes().to_vec()) {
                            Ok(t) => t,
                            Err(e) => return invalid(e),
                        };

                    debug!("response: {:?}", response);

                    let t = self.user_agent.clone();

                    if t == response.user_agent_browser {
                        // response is valid for five seconds
                        debug!(
                            "\n\nEpoch: {}, response.epoch: {}\n\n",
                            epoch(),
                            response.epoch
                        );
                        if epoch() - 10 <= response.epoch {
                            let tls_fingerprint = self.connection_context.fingerprint;

                            if let Some(fingerprint) = tls_fingerprint {
                                let browser_matched = match fingerprint {
                                    TlsFingerprint::Firefox => {
                                        GA.cgi.challenge_firefox.inc();
                                        response.browser == "Firefox"
                                    }
                                    TlsFingerprint::Chrome => {
                                        GA.cgi.challenge_chrome.inc();
                                        response.browser == "Chrome"
                                    }
                                    TlsFingerprint::Safari => {
                                        GA.cgi.challenge_safari.inc();
                                        response.browser == "Safari"
                                    }
                                    TlsFingerprint::Unknown => false,
                                };

                                debug!(
                                    "browser matched: {}, fingerprint: {:?}, browser: {}",
                                    browser_matched, fingerprint, response.browser
                                );

                                // now we know the user agents match, it's freshly created, the fingerprint matches the browser
                                // we're about to issue the cookie, but let's pencil in some statistics
                                if browser_matched {
                                    if response.audio_context {
                                        GA.cgi.challenge_audio_context.inc();
                                    }

                                    if !response.notifications_persistent
                                        && response.browser == "Chrome"
                                    {
                                        GA.cgi.challenge_notifications_not_persistent.inc();
                                    }

                                    if response.fixed_memory_set {
                                        GA.cgi.challenge_fixed_memory_set.inc();
                                    }

                                    if self.ip.allow(NewTrafficType::Token) {
                                        // now, add the token
                                        self.ip.add_token(Token {
                                            user_agent: self.user_agent.clone(),
                                            fingerprint: tls_fingerprint.unwrap(),
                                            points: Counter::new(),
                                        });

                                        GA.cgi.submit_challenge_success.inc();

                                        return resp("hello human", Some(StatusCode::OK), true);
                                    }

                                    /* no longer using cookies
                                                                       let cookie = ChallengeCookie {
                                                                           ip: self.ip.ip.clone(),
                                                                           exp: epoch() + 86400,
                                                                           website: self.domain.domain.clone(),
                                                                           tls_fingerprint: fingerprint,
                                                                           ua: self.user_agent.clone()
                                                                       };

                                                                       if path == "/__pw/submit-challenge" {
                                                                           self.domain.analytic.challenge_completed.inc();
                                                                       } else if path == "/__pw/bg" {
                                                                           self.domain.analytic.turbo_mode_completed.inc();
                                                                       }

                                                                       return resp(&cookie.create_cookie(), Some(StatusCode::OK), true);

                                    */
                                } else {
                                    GA.cgi.challenge_browser_no_match.inc();
                                }
                            } else {
                                GA.cgi.challenge_browser_no_match.inc();
                            }
                        } else {
                            GA.cgi.challenge_timeout_replay.inc();
                        }
                    } else {
                        GA.cgi.challenge_ua_no_match.inc();
                    }

                    GA.cgi.submit_challenge_fail.inc();
                    return resp(
                        "Invalid challenge response",
                        Some(StatusCode::UNPROCESSABLE_ENTITY),
                        true,
                    );
                }
                resp("404", Some(StatusCode::NOT_FOUND), true)
            }
            "/__pw/trace" => resp(
                &format!(
                    r##"ip: {ip},
user agent: {ua},
proxy_server: {server},
ts: {ts},
p: {p}"##,
                    ip = self.ip.ip,
                    ua = self.user_agent,
                    server = std::env::var("NODE_NAME").unwrap(),
                    ts = self.get_score(),
                    p = self.connection_context.convert_fingerprint_to_num()
                ),
                Some(StatusCode::OK),
                true,
            ),
            "/__pw/bg-js" => {
                let bg_js = BACKGROUND_CHALLENGE.read().unwrap();

                resp(bg_js.deref(), Some(StatusCode::OK), false)
            }
            _ => resp("404", Some(StatusCode::NOT_FOUND), true),
        }
    }
}

impl ConnectionContext {
    pub fn convert_fingerprint_to_num(&self) -> i32 {
        if let Some(t) = self.fingerprint {
            match t {
                TlsFingerprint::Firefox => 1,
                TlsFingerprint::Chrome => 2,
                TlsFingerprint::Safari => 3,
                TlsFingerprint::Unknown => 4,
            }
        } else {
            0
        }
    }
}
