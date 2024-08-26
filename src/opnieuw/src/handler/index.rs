use crate::{
    debug,
    handler::{
        models::ConnectionContext,
        well_known::handler::well_known_handler,
        PipelineFunction,
    },
    models::{
        analytics_by_example::AnalyticsByExample,
        pipeline_response::{
            PipelineResponse,
            Pipelines,
        },
        request_context::{
            PipelineData,
            RequestContext,
        },
    },
    rproxy::index::reverse_proxy,
    templates::{
        direct_ip::direct_ip_reject,
        domain_not_found::domain_not_found,
        error::internal_error,
        global_ratelimit::global_ratelimit_template,
    },
    utils::{
        domains::{
            get_user_agent,
            is_domain,
        },
        get_host::{
            get_host,
            get_host_db,
        },
        redirect::perform_redirect,
    },
    HttpResponse,
    CERTS,
    GA,
};
use anyhow::Result;
use http_body_util::{
    combinators::BoxBody,
    BodyExt,
    Full,
};
use hyper::{
    body::{
        Body,
        Bytes,
    },
    header::LOCATION,
    Request,
    Response,
};
use std::{
    cell::RefCell,
    net::IpAddr,
    rc::Rc,
    sync::atomic::AtomicBool,
};

lazy_static!(
    // a vector of Pipeline Function & the enum of the Pipeline
    // you might ask, why a tuple of this? this is so we can figure out which pipelines to skip.
    static ref PIPELINE: Vec<(PipelineFunction, Pipelines)> = vec![
        (|s: &RequestContext, data: &Vec<PipelineData>| -> PipelineResponse { s.request_inspection(data) }, Pipelines::RequestInspection),
        (|s: &RequestContext, data: &Vec<PipelineData>| -> PipelineResponse { s.api_engine(data) }, Pipelines::ApiEngine),
        (|s: &RequestContext, data: &Vec<PipelineData>| -> PipelineResponse { s.bot_management(data) }, Pipelines::VerifiedBots),
        (|s: &RequestContext, data: &Vec<PipelineData>| -> PipelineResponse { s.rules(data) }, Pipelines::Rules),
        (|s: &RequestContext, data: &Vec<PipelineData>| -> PipelineResponse { s.human_engine(data) }, Pipelines::HumanEngine),
        (|s: &RequestContext, data: &Vec<PipelineData>| -> PipelineResponse { s.cache_engine(data) }, Pipelines::Caching),
    ];
);

pub async fn handler_middleware(
    req: Request<hyper::body::Incoming>,
    conn_ctx: ConnectionContext,
) -> Result<HttpResponse, hyper::Error> {
    Ok(handler(req, conn_ctx).await)
}

pub async fn handler(
    req: Request<hyper::body::Incoming>, // the request data
    conn_ctx: ConnectionContext,         // connection context that includes data from the stream
) -> HttpResponse {
    // increment the global analytic for a new request coming in
    GA.requests.inc();
    let global_ratelimit_resp = conn_ctx.global_ratelimit();

    debug!("Request: {:?}, {:?}", req, conn_ctx);

    // check if the global ratelimit is blocking the request
    if let PipelineResponse::StopProcessing(resp) = global_ratelimit_resp {
        return resp;
    }

    // acme challenge
    if req.uri().path().starts_with("/.well-known/acme-challenge/") {
        GA.cgi.well_known.inc();

        let token = req.uri().path().replace("/.well-known/acme-challenge/", "");

        if let PipelineResponse::StopProcessing(resp) = well_known_handler(token) {
            return resp;
        }
    }

    // get the host from either the URI or the Host header
    let raw_host = match req.uri().host() {
        Some(t) => t.to_string(),
        None => {
            if !conn_ctx.http2 {
                // the only way the URI doesn't have the Host is if we're running HTTP/1.X
                match req.headers().get("Host") {
                    Some(t) => match t.to_str() {
                        Ok(t) => {
                            if !is_domain(t) {
                                return direct_ip_reject();
                            } else {
                                t.to_string()
                            }
                        }
                        Err(_) => return internal_error("Couldn't unwrap host!"),
                    },
                    None => return direct_ip_reject(),
                }
            } else {
                return direct_ip_reject();
            }
        }
    };

    let host = raw_host.as_str();

    debug!("host: {}", host);

    // now that we have the host, if we have a certificate for it, we can redirect to HTTPS
    if !conn_ctx.https && CERTS.get(host).is_some() {
        return perform_redirect(&format!("https://{}{}", host, req.uri()));
    }

    /*
    The third thing we want to do is to get all relevant data, because actions from here will be domain-specific
    */

    // why do we make this reference here? because now the lifetime exists for the entire duration of the proxy
    // if we created this in the `RequestContext` block then we would unfortunately be dropping the value before
    // it can be used!
    //let mut additional_ctx = AdditionalContext { WSBucket: None };

    // get the user agent from the URI or the Host header
    let user_agent = get_user_agent(&req).unwrap_or_default();
    // split the request to parts and body
    let (parts, body) = req.into_parts();

    // create the request context by consuming the request and other vital pieces of data
    let context: RequestContext = RequestContext {
        req: parts, // this clone doesn't clone the whole obj ... just the pointer
        // the ip data
        ip: conn_ctx.ip.clone(),
        // the domain data
        domain: {
            match get_host_db(host) {
                Some(t) => t,
                None => return domain_not_found(),
            }
        },
        // the user agent, gotten from the header
        user_agent, // by default, creates ""
        // whether incoming streams should be consumed or passed on to the backend
        stream_allowed: AtomicBool::new(true), // TODO implement tmr morning
        // the full host, gotten from the URI or the header
        full_host: host.to_string(),
        // analytics by example (for the domain)
        by_example: RefCell::new(None),
        // the connection context
        connection_context: conn_ctx,
    };

    context
        .by_example
        .replace(AnalyticsByExample::new(&context));

    if context.req.uri.path().starts_with("/__pw/") {
        return context.cgi(body).await;
    }

    /*
    Now, we begin the processing of the pipeline! Good luck, request!
     */

    context.domain.analytic.total.inc();

    let mut to_skip: Vec<Pipelines> = Vec::new(); // Pipelines to be skipped!
    let mut req_data: Vec<PipelineData> = Vec::new(); // where additional data is saved

    'outer: for f in PIPELINE.iter() {
        crate::debug!("pipeline {:?}", f.1);
        // we only want to run this pipeline function if it's not been skipped
        if !to_skip.iter().any(|pipeline| pipeline == &f.1) {
            // using any instead to simplify
            match f.0(&context, &req_data) {
                // f is a tuple, so f.0 grants us the first entry
                PipelineResponse::Ok(d) => {
                    if let Some(t) = d {
                        for i in t {
                            req_data.push(i.clone())
                        }
                    }
                } // everything is okay!
                PipelineResponse::SkipPipeline(skipped, d) => {
                    if let Some(t) = d {
                        for i in t {
                            req_data.push(i.clone())
                        }
                    }

                    for i in skipped {
                        if i == Pipelines::All {
                            break 'outer;
                        }
                        to_skip.push(i) // otherwise, add the pipelines we should skip to the vec
                    }
                }
                PipelineResponse::StopProcessing(resp) => {
                    // we're returning a response right here & now
                    // the pipeline is responsible for building the body and status
                    return resp;
                }
                PipelineResponse::Error(error) => {
                    return internal_error(
                        format!("error serving {:#?} pipeline: {}", f.1, error).as_str(),
                    );
                }
            }
        }
    }

    reverse_proxy(context, req_data, body).await
}
