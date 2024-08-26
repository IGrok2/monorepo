// respond with HIT if we respond to hit here, then respond with either MISS or INVALID as we get response back from backend
// to determine how cachable the resource is

use crate::{
    cache_system::reader::CacheReader,
    handler::pipeline::{
        caching::models::CacheLevel,
        human_engine::smart_challenge::CookieTester,
    },
    models::{
        pipeline_response::PipelineResponse,
        request_context::{
            PipelineData,
            RequestContext,
        },
    },
    EGRESS_HEADERS,
    GA,
};
use http_body_util::BodyExt;
use hyper::{
    header::CONTENT_LENGTH,
    http::request::Builder,
    Response,
    StatusCode,
};
use std::time::Instant;
// use crate::rproxy::pipeline::compression::compress_zip;
use crate::{
    templates::error::internal_error,
    utils::resp::add_headers,
};

impl RequestContext {
    // TODO: fix cache miss header on egress
    // TODO if pipelined data says no cache, no cache
    pub fn cache_engine(&self, data: &[PipelineData]) -> PipelineResponse {
        if !self.domain.caching_settings.enabled
            || self.is_ws()
            || !self.domain.internal_settings.can_cache
        {
            // don't cache websockets
            return PipelineResponse::Ok(None);
        }

        let mut level = self.domain.caching_settings.default_cache_level; // null means not set

        for i in data {
            // we want to respect this here too
            if let Some(t) = i.cache_level {
                level = t.0
            }
        }

        if level != CacheLevel::Null && level != CacheLevel::None {
            if let Some(t) = self
                .domain
                .caching_settings
                .bucket
                .map
                .get(&(self.full_host.clone() + self.req.uri.path()))
            {
                GA.handler.c.found.inc();

                if t.lives_until > Instant::now() {
                    // entry hasn't expired, we can serve it!
                    // ok, so we have it back -- now we need to get some data to see if we can inject the challenge
                    // determine if this can be turbo moded
                    let mut turbo_mode = false;
                    let cookie_tester = self.token_tester_internal();

                    // test the headers
                    if self.domain.human_engine_settings.turbo_mode_enabled {
                        if let Some(t) = t.headers.get("content-type") {
                            if t == "text/html" {
                                match cookie_tester {
                                    CookieTester::Good => {
                                        turbo_mode = false;
                                    }
                                    CookieTester::Bad | CookieTester::NoCookie => {
                                        turbo_mode = true;
                                    }
                                }
                                if cookie_tester != CookieTester::Good {
                                    turbo_mode = true;
                                }
                            }
                        }
                    }

                    // create the builder
                    let mut builder = Response::builder().status(t.http_status);

                    // get the headers
                    let headers = builder.headers_mut().unwrap();

                    // by default, no encoding
                    // let mut encoding = false;

                    // for if the content length header was able to be set without concern
                    let mut set_content_length = false;

                    // add the headers
                    for (k, v) in t.headers.iter() {
                        if turbo_mode && k.to_string().to_uppercase() == "CONTENT-LENGTH" {
                            if let Ok(t) = v.to_str() {
                                // okay, it could be parsed as a string, now let's make sure to add to the content-length header for turbo mode
                                if let Ok(val) = t.parse::<u64>() {
                                    // okay, it was an integer, so add on
                                    headers.insert(
                                        CONTENT_LENGTH,
                                        (val + 35).to_string().parse().unwrap(),
                                    );

                                    set_content_length = true;
                                }
                            }

                            if !set_content_length {
                                // failed setting
                                headers.insert(k, v.clone());
                            }
                        } else {
                            headers.insert(k, v.clone());
                        }
                    }

                    /* Going to test with continued setting
                    if !set_content_length {
                        turbo_mode = false;
                    }

                     */

                    // inject the first headers so we can overwrite if needs be

                    for i in EGRESS_HEADERS.iter() {
                        headers.insert(&i.0, i.1.parse().unwrap());
                    }

                    // add the cache header
                    headers.insert("pw-cache", "hit".parse().unwrap());

                    /*
                    TODO: encoding with caching

                    // check if we can encode the resource
                    if cookie_tester == CookieTester::Good && self.can_compress(&t.headers) {
                        // our checks of the request response and their headers suggest it is possible to encode this resource
                        encoding = true;

                        // add content-encoded header to response
                        builder.insert_header(("Content-Encoding", "gzip"))
                            .insert_header(("Encoded-By", "pw"));

                        use std::ops::DerefMut; if let Some(ref mut t) = self.by_example.borrow_mut().deref_mut() {
                            t.caching.encoded = true;
                        }
                    }

                     */

                    GA.handler.c.cache_hit.inc();

                    self.domain.analytic.cached_reqs.inc();

                    let cache_reader = CacheReader::new(t.value(), turbo_mode);

                    if let Some(reader) = cache_reader {
                        use std::ops::DerefMut;
                        if let Some(ref mut b) = self.by_example.borrow_mut().deref_mut() {
                            b.caching.bytes_written = t.size as u64 / 1000;
                            b.caching.hit = true;
                            b.proxy.response_code = Some(u16::from(t.http_status))
                        }

                        return PipelineResponse::StopProcessing(
                            builder.body(reader.boxed()).unwrap(),
                        );
                    }
                } else {
                    GA.handler.c.expired.inc();
                }
            }
        }

        PipelineResponse::Ok(None) // we will add the special headers on the egress
    }
}
