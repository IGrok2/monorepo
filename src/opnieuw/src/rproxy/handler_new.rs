use crate::{
    cache_system::writer::CacheWriter,
    debug,
    handler::pipeline::caching::models::CacheLevel,
    models::{
        domain_context::OriginSetting,
        egress_wrapper::EgressWrapper,
        request_context::{
            PipelineData,
            RequestContext,
        },
    },
    rproxy::{
        outbound_wrapper::OutboundWrapper,
        pipeline::utils::get_content_length,
        tools::tls_config::tls_config,
    },
    tls::cert_roots::webpki_roots_cert_store,
    utils::resp::add_headers,
    HttpResponse,
    EGRESS_HEADERS,
    GA,
};
use futures_util::task::Spawn;
use http_body_util::BodyExt;
use hyper::{
    body::Incoming,
    header::HeaderValue,
    http::request::Builder,
    Response,
};
use hyper_util::rt::TokioIo;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_rustls::{
    rustls::ClientConfig,
    TlsConnector,
};
use url::Url;

#[rustfmt::skip]
lazy_static! {
    static ref TLS_CLIENT: Arc<ClientConfig> = tls_config();
}

pub async fn call_backend(
    ctx: RequestContext,
    data: Vec<PipelineData>,
    origin: (Arc<OriginSetting>, (Url, Option<String>)),
    payload: Incoming,
) -> HttpResponse {
    // make the request to the client backend

    // build the new URL path to the origin
    let mut new_url: Url = origin.1 .0.clone(); // we need to clone because we're going to make some modifications
    new_url.set_path(ctx.req.uri.path());
    new_url.set_query(ctx.req.uri.query());

    // now, get what we need for the TcpStream
    let mut host = match new_url.host_str() {
        Some(t) => t.to_string(),
        None => return ctx.origin_invalid(),
    };

    // now get the port
    match new_url.port() {
        Some(t) => {
            host = format!("{}:{}", host, t);
        }
        None => {
            if new_url.scheme() == "https" {
                host = format!("{}:443", host);
            } else if new_url.scheme() == "http" {
                host = format!("{}:80", host);
            } else {
                return ctx.origin_invalid();
            }
        }
    }

    // pretend host
    if let Some(pretend_host) = origin.1 .1.clone() {
        new_url.set_host(Some(pretend_host.as_str())).unwrap();
    }

    // now that we've sourced the origin from the pipeline data and domain settings, let's create a client to reach it
    let mut req = Builder::new()
        .method(ctx.req.method.clone())
        .uri(new_url.as_str())
        .version(ctx.req.version);

    // now that we've got the request, let's get the header map so we can add the headers
    if let Some(headers) = req.headers_mut() {
        for (key, value) in ctx.req.headers.iter() {
            headers.insert(key, value.clone());
        }

        // now, add the ip addresses to the headers
        let ip = ctx.ip.ip.to_string();

        headers.insert(
            "X-Forwarded-For",
            ip.parse().unwrap_or("ERROR".parse().unwrap()),
        );
        headers.insert("X-Real-IP", ip.parse().unwrap_or("ERROR".parse().unwrap()));
        headers.insert(
            "Pw-Connecting-IP",
            ip.parse().unwrap_or("ERROR".parse().unwrap()),
        );
        headers.insert(
            "CF-Connecting-IP",
            ip.parse().unwrap_or("ERROR".parse().unwrap()),
        );
        headers.insert("Host", new_url.host().unwrap().to_string().parse().unwrap());

        // now, add the egress headers
        add_headers(headers);

        // TODO: if we continue using cookies, make sure to remove the __pw_loves_you cookie

        // now, if turbo mode is enabled, there's no encoding
        if ctx.domain.human_engine_settings.turbo_mode_enabled {
            headers.insert("Accept-Encoding", HeaderValue::from_static(""));
        }

        // now, if we need ip data, let's get it
        if origin.0.ip_data {
            let ip_data = ctx.get_ipdata();
            // update rproxy counter
            GA.rproxy.ip_data_requested.inc();

            // now, add the ip data to the headers
            headers.insert(
                "X-Country",
                ip_data.0.parse().unwrap_or("ERROR".parse().unwrap()),
            );
            headers.insert(
                "X-Continent",
                ip_data.1.parse().unwrap_or("ERROR".parse().unwrap()),
            );
            headers.insert(
                "X-ASN",
                ip_data.2.parse().unwrap_or("ERROR".parse().unwrap()),
            );
        }
    } else {
        return ctx.origin_invalid();
    }

    // here's where we'll depend on the responses, so timeouts will be critical

    // first get the domain so we can move it safely
    let domain = ctx.domain.clone();

    let context = EgressWrapper::new(&ctx);

    // get the ip so we can get the abort handle
    let ip = ctx.ip.clone();

    // now that the request is finalized, append the body
    let finished_req = req.body(payload).unwrap();

    // then run the timeout
    #[allow(clippy::blocks_in_conditions)]
    match domain
        .clone()
        .stream_timeout(origin.0.timeout, async move {
            debug!("Connecting to origin: {}", host);

            // create the TcpStream
            let stream = match TcpStream::connect(host).await {
                Ok(t) => t,
                Err(e) => {
                    debug!("Error connecting to origin: {}", e);

                    return Err(format!("{:?}", e));
                }
            };

            // find out if we need to use tls
            match origin.0.ssl {
                true => {
                    let tls_connector = TlsConnector::from(TLS_CLIENT.clone());

                    let tls_stream = tls_connector
                        .connect(
                            tokio_rustls::rustls::ServerName::try_from(
                                new_url.host().unwrap().to_string().as_str(),
                            )
                            .unwrap(),
                            stream,
                        )
                        .await
                        .unwrap();

                    // finish tls cert

                    // now, we need to send the request to the origin
                    let (mut request_sender, connection) =
                        hyper::client::conn::http1::handshake(TokioIo::new(tls_stream))
                            .await
                            .unwrap();

                    // send the request
                    debug!("Sending request to origin: {:?}", request_sender);

                    let task = tokio::task::spawn(async move {
                        let _ = connection.await;
                    });

                    // add to the abort handles
                    ip.add_handle(task.abort_handle());

                    debug!("Connection established to origin: {:?}", request_sender);
                    let resp = request_sender
                        .send_request(finished_req)
                        .await
                        .map_err(|e| format!("{:?}", e))?;

                    // now, we need to send the response back to the client
                    debug!("Received response from origin: {:?}", resp);

                    Ok(resp)
                }
                false => {
                    // now, we need to send the request to the origin
                    let (mut request_sender, connection) =
                        hyper::client::conn::http1::handshake(TokioIo::new(stream))
                            .await
                            .unwrap();

                    // send the request
                    debug!("Sending request to origin: {:?}", request_sender);

                    let task = tokio::task::spawn(async move {
                        let _ = connection.await;
                    });

                    // add to the abort handles
                    ip.add_handle(task.abort_handle());

                    debug!("Connection established to origin: {:?}", request_sender);
                    let resp = request_sender
                        .send_request(finished_req)
                        .await
                        .map_err(|e| format!("{:?}", e))?;

                    // now, we need to send the response back to the client
                    debug!("Received response from origin: {:?}", resp);

                    Ok(resp)
                }
            }
        })
        .await
    {
        Ok(t) => {
            match t {
                Ok(resp) => {
                    // now, we build the response
                    let mut builder = Response::builder()
                        .status(resp.status())
                        .version(resp.version());

                    // get the headers from the builder so we can start changing them
                    let headers = builder.headers_mut().unwrap();

                    // add the headers from the response from the backend
                    for (key, value) in resp.headers().iter() {
                        headers.insert(key, value.clone());
                    }

                    // get turbo mode status
                    let turbo_mode = ctx.get_turbo(&resp, headers);

                    // match the cache status
                    let cache_status =
                        match ctx.should_cache(&resp, &data, &ctx.req.uri.to_string()) {
                            (CacheLevel::Null, _) | (CacheLevel::None, _) => {
                                debug!("not caching");
                                None
                            }
                            (level, mut ttl) => {
                                debug!("decided to cache: {:?}", level);

                                // this path is about to be cached
                                GA.rproxy.attempting_cache.inc();

                                headers.insert("pw-cache", "attempting".parse().unwrap());

                                if ttl.is_none() {
                                    ttl = Some(ctx.domain.caching_settings.default_cache_ttl)
                                }

                                CacheWriter::new(
                                    ctx.domain.clone(),
                                    // TODO change TTL back
                                    ttl.unwrap(),
                                    ctx.full_host.to_string() + ctx.req.uri.path(),
                                    resp.headers().clone(),
                                    resp.status(),
                                    get_content_length(&resp),
                                )
                                .await
                            }
                        };

                    // add the egress headers
                    add_headers(headers);

                    use std::ops::DerefMut;
                    if let Some(ref mut t) = ctx.by_example.borrow_mut().deref_mut() {
                        t.proxy.hit = true;
                        t.proxy.response_code = Some(resp.status().as_u16());
                        t.proxy.origin_setting_ip = origin.1 .0.to_string();
                    }

                    builder
                        .body(
                            OutboundWrapper::new(
                                resp.into_body(),
                                cache_status,
                                context,
                                turbo_mode,
                            )
                            .boxed(),
                        )
                        .unwrap()
                }
                Err(e) => {
                    use std::ops::DerefMut;
                    if let Some(ref mut t) = ctx.by_example.borrow_mut().deref_mut() {
                        t.proxy.hit = true;
                        t.proxy.errored = Some(e);
                        t.proxy.origin_setting_ip = origin.1 .0.to_string();
                    }

                    context.ctx.origin_down()
                }
            }
        }
        Err(e) => {
            debug!("Timeout connecting to origin: {}", e);

            context.ctx.origin_down()
        }
    }
}
