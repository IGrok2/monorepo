use crate::{HttpResponse, EGRESS_HEADERS};
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::header::HeaderValue;
use hyper::http::response::Builder;
use hyper::{HeaderMap, Response, StatusCode};
use std::convert::Infallible;

pub fn resp(body: &str, status: Option<StatusCode>, no_store: bool) -> HttpResponse {
    let mut builder = Response::builder().status(status.unwrap_or(StatusCode::IM_A_TEAPOT));

    let headers = builder.headers_mut().unwrap();

    for i in EGRESS_HEADERS.iter() {
        headers.insert(i.0.as_str(), i.1.as_str().parse().unwrap());
    }

    if no_store {
        headers.insert("Cache-Control", "no-store".parse().unwrap());
        headers.insert("Connection", "close".parse().unwrap());
    }

    builder
        .body(Full::new(Bytes::from(body.to_string())).boxed())
        .unwrap()
}

pub fn add_headers(headers: &mut HeaderMap) {
    // elided lifetimes
    for (_n, i) in EGRESS_HEADERS.iter().enumerate() {
        // thanks to .enum, n is the index and i is the value
        // before, when we set the value to it, we needed this to return it, now we don't!
        // if n == EGRESS_HEADERS.len() - 1 { // last entry
        //     headers.insert(i.0.as_str(), HeaderValue::from_str(i.1.as_str()).unwrap());
        // }

        headers.insert(i.0.as_str(), HeaderValue::from_str(i.1.as_str()).unwrap());
    }

    // panic!("Egress headers was none"); // ultimate problem solver
}
