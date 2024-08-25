use crate::utils::resp::add_headers;
use crate::{HttpResponse, EGRESS_HEADERS};
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::{Response, StatusCode};

pub fn perform_redirect(to: &str) -> HttpResponse {
    let mut resp = Response::builder().status(StatusCode::TEMPORARY_REDIRECT);

    add_headers(resp.headers_mut().unwrap()); //
    resp.headers_mut()
        .unwrap()
        .insert("Location", to.parse().unwrap());

    resp.body(Full::new(Bytes::from("Redirecting ...")).boxed())
        .unwrap()
}
