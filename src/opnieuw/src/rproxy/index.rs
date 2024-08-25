use crate::models::request_context::RequestContext;
use crate::{HttpResponse, GA};
use hyper::body::Incoming;
// use crate::handler::pipeline::api_engine::ws::handler::handle_websocket;
use crate::models::request_context::PipelineData;
use crate::rproxy::handler_new::call_backend;

//use crate::rproxy::handler::call_backend;
lazy_static!(
    // we can use "unwrap" here because we want to propogate any potential errors - plus we are running this once (it should never panic)
    // static ref WS_HEADER: HeaderValue = HeaderValue::from_str("websocket").unwrap();
);
// congratulations, the request has passed our system! time to make sure everything is all good
// and then toss it over to their backend
pub async fn reverse_proxy(
    context: RequestContext,
    data: Vec<PipelineData>,
    payload: Incoming,
) -> HttpResponse {
    /*
    Now that everything has passed our check system, it will take the following path before reaching
    the backend:
    1. Check some qualities about the backend (make sure it's not a local address, and also make sure
            that it's not pointed at another DCDN zone)
    2. Acquire a slot to contact this backend (if not, show too many requests for this site)
        Check the lock for this path by storing a Lock in a ConcurrentMap, which depends on their caching choice:
            a) If an object hasn't been requested yet, stream the response to multiple client bodies
            b) Let the client bodies go through
    3. Try to dial the backend
        return an error if it doesn't work
        count the bandwidth usage to their total quota
    4. See if we can stream the response into the DB
     */

    // First, let's do some checks to make sure everything is kosher!

    let origin = match context.choose_backend(&data) {
        Some(t) => {
            GA.rproxy.origin_found.inc();
            t.0.hits.inc();
            t
        }
        None => {
            GA.rproxy.origin_not_found.inc();
            return context.origin_invalid();
        }
    };

    if origin.0.is_localhost {
        // make sure we're not proxying to localhost addresses
        GA.rproxy.localhost.inc();
        context.domain.analytic.invalid_reqs.inc();
        return context.origin_invalid();
    }

    // now, we're ready to send this to the backend, so let's acquire a slot to contact the backend
    if context.domain.origin.open_conns.get()
        > context.domain.internal_settings.allowed_open_conns as i64
    {
        GA.rproxy.rl_backend.inc();
        return context.too_many_requests_for_backend();
    }

    /* TODO - we need to handle websockets with hyper
    // if it's a websocket
    if context.is_ws() {
        GA.rproxy.is_ws.inc();

        let mut methods = None;
        let mut bucket = None;

        crate::debug!("data: {:?}", data.len());

        for i in data.iter() {
            if let Some(t) = i.ws_methods.clone() {
                methods = Some(t);
            }

            if let Some(t) = i.bucket.clone() {
                bucket = Some(t);
            }
        }

        // we would want it to panic if it doesn't work out
        crate::debug!("methods: {:?}, bucket: {:?}", methods.is_some(), bucket);

        return match methods {
            Some(t) => handle_websocket(context, origin, bucket, t, &data, payload).await,
            None => context.api_engine_required()
        }
    }

     */

    // we no longer handle websockets here, so we can just call the backend
    call_backend(context, data, origin, payload).await
}
