use hyper::{
    http::request::Parts,
    Request,
};
use std::{
    cell::RefCell,
    net::Ipv4Addr,
    rc::Rc,
    sync::{
        atomic::AtomicBool,
        Arc,
    },
    time::Duration,
};

use crate::{
    buckets::models::PublicBucket,
    handler::{
        models::ConnectionContext,
        pipeline::{
            api_engine::models::WsMethods,
            caching::models::CacheLevel,
        },
    },
    ip::models::IP,
    models::{
        analytics_by_example::AnalyticsByExample,
        domain_context::{
            DomainContext,
            OriginSetting,
            OriginType,
        },
    },
};

pub struct RequestContext {
    // lifetime name is 'a
    pub req: Parts,
    pub domain: Arc<DomainContext>, // this obj will last as long as the struct does
    pub ip: Arc<IP>,
    pub user_agent: String,
    pub stream_allowed: AtomicBool, // is a stream allowed? let's let the API engine decide ...
    pub full_host: String,          // the full host being requested
    pub by_example: RefCell<Option<AnalyticsByExample>>,
    pub connection_context: ConnectionContext, // the connection context
                                               // pub payload: Arc<Payload>
                                               //pub more: &'a mut AdditionalContext,
                                               //pub resp: Cell<HttpResponseBuilder> // response object, this is not used by handler
                                               // must use Cell because Rust does not support field mutability, so we have to use interior mutability
                                               // Cell supports .set() and .get()
}

#[derive(Clone)]
pub struct PipelineData {
    // additional context that has been added onto this request, might be helpful
    pub cache_level: Option<(CacheLevel, Duration)>, // duration + cache level to be used here
    pub bucket: Option<Arc<PublicBucket>>,           // bucket used for websockets
    pub ws_methods: Option<Vec<WsMethods>>,          // allowed Websocket methods
    pub backend: Option<Arc<OriginType>>,            // the backend for this request
}
