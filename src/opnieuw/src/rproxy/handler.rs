use actix_http::{
    header::HeaderName,
    Version,
};
use actix_web::http::header::HeaderValue;
use std::{
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use actix_web::web::Payload;
use http_body_util::BodyStream;
use hyper::http::request::Builder;
use tokio::net::TcpStream;

use awc::body::BodyStream;

use awc::error::SendRequestError;
use rustls::ClientConfig;

use crate::{
    cache_system::writer::CacheWriter,
    debug,
    handler::pipeline::{
        caching::models::CacheLevel,
        human_engine::smart_challenge::CookieTester,
    },
    models::{
        domain_context::OriginSetting,
        egress_wrapper::EgressWrapper,
        request_context::{
            PipelineData,
            RequestContext,
        },
    },
    tools::resp::add_headers,
    vars::vars::{
        NODE_NAME,
        VERSION,
    },
    EGRESS_HEADERS,
    GA,
    ORDER,
};
use actix_web::{
    HttpResponse,
    HttpResponseBuilder,
};
use awc::Client;
use url::Url;
//use crate::rproxy::inbound_wrapper::InboundWrapper;
use crate::{
    rproxy::{
        outbound_wrapper::OutboundWrapper,
        pipeline::utils::get_content_length,
    },
    ssl::cert_roots::webpki_roots_cert_store,
    utils::counter::Counter,
};

pub async fn call_backend(
    // ring it up!
    context: RequestContext,
    origin: (Arc<OriginSetting>, (Url, Option<String>)),
    data: Vec<PipelineData>,
    payload: Payload,
) -> HttpResponse {
    context.domain.analytic.proxied_reqs.inc();

    let host = match origin.1 .1 {
        Some(t) => t.clone(),
        None => context.full_host.clone(),
    };

    // build the new URL path to the origin
    let mut new_url: Url = origin.1 .0.clone(); // we need to clone because we're going to make some modifications
    new_url.set_path(context.req.uri.path());
    new_url.set_query(context.req.uri.query());

    let port = origin.1 .0.port().unwrap_or(80);

    let stream = TcpStream::connect((origin.1 .0));

    // Build the http version
    let (mut sender, conn) = Builder::new()
        .method(context.req.method.clone())
        .uri(new_url.as_str())
        .version(context.req.version())
        .body(BodyStream::new(payload))
        .unwrap()
        .into_parts();

    let version;

    match origin.0.http2 {
        true => version = Version::HTTP_2,
        false => version = Version::HTTP_11,
    };

    let mut prepped_res = match origin.0.ssl {
        true => {
            let mut config = ClientConfig::builder()
                .with_safe_defaults()
                .with_root_certificates(webpki_roots_cert_store())
                .with_no_client_auth();

            config
                .dangerous()
                .set_certificate_verifier(Arc::new(danger::NoCertificateVerification));

            if origin.0.http2 {
                config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
            }

            Client::builder()
                .disable_redirects()
                .connector(awc::Connector::new().rustls_021(Arc::new(config)))
                .finish()
                .request_from(new_url.as_str(), context.req.head()) // building off this "base"
                .timeout(origin.0.timeout) // set the timeout for this request
                .insert_header(("X-Forwarded-For", context.ip.as_str())) // set the X-Forwarded-For header, this overrides any headers the client sends
                .insert_header(("X-Real-IP", context.ip.as_str()))
                .insert_header(("Pw-Connecting-IP", context.ip.as_str()))
                .insert_header(("Cf-Connecting-IP", context.ip.as_str()))
                .insert_header(("Host", host))
        }
        false => {
            Client::builder()
                .disable_redirects()
                .finish()
                .request_from(new_url.as_str(), context.req.head()) // building off this "base"
                .timeout(origin.0.timeout) // set the timeout for this request
                .insert_header(("X-Forwarded-For", context.ip.as_str())) // set the X-Forwarded-For header, this overrides any headers the client sends
                .insert_header(("X-Real-IP", context.ip.as_str()))
                .insert_header(("Pw-Connecting-IP", context.ip.as_str()))
                .insert_header(("Cf-Connecting-IP", context.ip.as_str()))
                .insert_header(("Host", host))
        }
    };

    // remove challenge cookie if it exists
    if let Some(t) = prepped_res.headers_mut().get_mut("Cookie") {
        if let Ok(cookie) = t.to_str() {
            if let Some(start_index) = cookie.find("__pw_loves_you=") {
                if let Some(end_index) = cookie[start_index..].find(";").map(|i| start_index + i) {
                    if let Ok(mut ending_value) = HeaderValue::from_str(
                        &(cookie[..start_index].to_string() + &cookie[end_index..]),
                    ) {
                        *t = ending_value;
                    }
                }
            }
        }
    };

    for (k, v) in EGRESS_HEADERS.iter() {
        prepped_res = prepped_res.insert_header((k.as_str(), v.as_str()));
    }

    if context.domain.human_engine_settings.turbo_mode_enabled {
        // if we're going to be inserting the challenge, we can't have any encoding on the response
        prepped_res = prepped_res.insert_header(("Accept-Encoding", ""));
    }

    if origin.0.ip_data {
        // they have ip data enabled
        let ip_data = context.get_ipdata();
        GA.rproxy.ip_data_requested.inc();

        prepped_res = prepped_res // commit the move so we can borrow the value lower when we check for the stream
            .insert_header(("X-Country", ip_data.0))
            .insert_header(("X-Continent", ip_data.1))
            .insert_header(("X-ASN", ip_data.2));
    }

    let sent_res;

    GA.rproxy.sent_to_backend.inc();
    let ctx = EgressWrapper::new(context);

    if ctx.ctx.stream_allowed.load(ORDER) {
        // if the api engine indicated that it was OK to send data here
        GA.rproxy.stream_allowed.inc();
        sent_res = prepped_res.send_stream(payload).await;
    } else {
        GA.rproxy.stream_denied.inc();
        sent_res = prepped_res.send().await;
    }

    let res;

    match sent_res {
        Ok(client) => {
            use std::ops::DerefMut;
            if let Some(ref mut t) = ctx.ctx.by_example.borrow_mut().deref_mut() {
                t.proxy.hit = true;
                t.proxy.response_code = Some(client.status().as_u16());
                t.proxy.origin_setting_ip = origin.1 .0.to_string();
            }

            res = client
        }
        Err(error) => {
            use std::ops::DerefMut;
            if let Some(ref mut t) = ctx.ctx.by_example.borrow_mut().deref_mut() {
                t.proxy.hit = true;
                t.proxy.errored = Some(format!("{:?}", error));
                t.proxy.origin_setting_ip = origin.1 .0.to_string();
            }

            match error {
                SendRequestError::Url(_) => {}
                SendRequestError::Connect(_) => {}
                SendRequestError::Send(_) => {}
                SendRequestError::Response(_) => {}
                SendRequestError::Http(_) => {}
                SendRequestError::H2(_) => {}
                SendRequestError::Timeout => {}
                SendRequestError::TunnelNotSupported => {}
                SendRequestError::Body(_) => {}
                SendRequestError::Custom(_, _) => {}
                _ => {}
            }
            debug!("error calling backend: {:?}", error);
            return ctx.ctx.origin_down();
        }
    };

    // now, finally, we clear out some headers, and tee off the response structure

    // determine if this can be turbo moded
    let mut turbo_mode = false;
    let cookie_tester = ctx.ctx.token_tester_internal();

    if ctx.ctx.domain.human_engine_settings.turbo_mode_enabled {
        if let Some(t) = res.headers().get("content-type") {
            if t == "text/html" {
                if cookie_tester != CookieTester::Good {
                    turbo_mode = true;
                }
            }
        }
    }

    let cache_key = ctx.ctx.full_host.clone() + ctx.ctx.req.path();

    let mut ttl = None;

    let content_length = get_content_length(&res);

    let mut cache_level = ctx
        .ctx
        .should_cache(&res, &data, &cache_key, content_length);

    debug!(
        "cache level response: {:?}, {:?}",
        cache_level.0, cache_level.1
    );

    let mut resp = HttpResponseBuilder::new(res.status());

    for (i, v) in res.headers() {
        // if the response has set-cookie, then don't cache
        if i.to_string().to_uppercase() == "SET-COOKIE" {
            cache_level.0 = CacheLevel::Null;
        }

        resp.insert_header((i, v));
    }

    let mut encoding = false;

    /* TODO: fix encoding not working on safari
    // check if we can encode the resource
    if cookie_tester == CookieTester::Good && ctx.ctx.can_compress(res.headers()) {
        // our checks of the request response and their headers suggest it is possible to encode this resource
        encoding = true;

        // add content-encoded header to response
        resp.insert_header(("Content-Encoding", "gzip"))
            .insert_header(("Encoded-By", "pw"));
    }

     */

    // if we need to modify the content-length
    let content_length = get_content_length(&res);

    if turbo_mode {
        if content_length > 0 {
            // then, change the header to allow for more content length
            resp.insert_header(("Content-Length", content_length + 35));
        }
    }

    ctx.ctx
        .domain
        .analytic
        .data_transferred_outbound
        .inc_by(res.headers().len() as i64 * 10);

    // these should only be set if it's being cached!

    let status = res.status();
    let mut headers = res.headers().clone();
    let mut cache_allocation = None;

    if content_length != 0 {
        match cache_level.0 {
            CacheLevel::Standard | CacheLevel::Aggressive | CacheLevel::IgnoreQueryString => {
                // TODO: make sure request doesn't have a query string
                let mut caching_confirmed = false;

                // see if it's too big to be individually cached
                if content_length / 1000 > ctx.ctx.domain.internal_settings.cache_file_max as u64 {
                    GA.rproxy.cache_max_hit.inc();

                    cache_level = (CacheLevel::Null, None);
                }

                // check the total store
                if ctx.ctx.domain.caching_settings.bucket.total_size.get()
                    + (content_length / 1000) as i64
                    > ctx.ctx.domain.internal_settings.total_cache_limit as i64
                {
                    GA.rproxy.cache_max_hit.inc();

                    cache_level = (CacheLevel::Null, None);
                } else {
                    caching_confirmed = true;
                }

                if caching_confirmed {
                    // this path is about to be cached
                    GA.rproxy.attempting_cache.inc();
                    (&mut resp).insert_header(("pw-cache", "attempting"));
                    ttl = cache_level.1;

                    if ttl.is_none() {
                        ttl = Some(ctx.ctx.domain.caching_settings.default_cache_ttl)
                    }

                    cache_allocation = CacheWriter::new(
                        ctx.ctx.domain.clone(),
                        // TODO change TTL back
                        ttl.unwrap(),
                        (ctx.ctx.full_host.clone() + ctx.ctx.req.path()),
                        headers,
                        status,
                        content_length,
                    )
                    .await;
                } else {
                    (&mut resp).insert_header(("pw-cache", "blown"));
                }
            }
            _ => {}
        }
    }

    debug!("calling rproxy: {:?}, {:?}", cache_level.0, cache_level.1);

    debug!("encoding status: {}", encoding);

    add_headers(&mut resp).body(OutboundWrapper::new(
        BodyStream::new(res),
        Counter::new(),
        ctx,
        cache_level.0,
        Some(cache_key),
        ttl,
        content_length,
        turbo_mode,
        encoding,
        cache_allocation,
    ))
}

// section for our certificate validator
mod danger {
    use super::*;
    use rustls::client::ServerCertVerifier;
    use std::time::SystemTime;

    pub struct NoCertificateVerification;

    // we don't care! (not yet)
    impl ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &rustls::Certificate,
            _intermediates: &[rustls::Certificate],
            _server_name: &rustls::ServerName,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _oscp_response: &[u8],
            _now: SystemTime,
        ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
            Ok(rustls::client::ServerCertVerified::assertion())
        }
    }
}
