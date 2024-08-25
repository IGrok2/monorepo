// get host, either from the host header or the uri

use crate::models::domain_context::DomainContext;
use crate::templates::direct_ip::direct_ip_reject;
use crate::templates::error::internal_error;
use crate::utils::domains::is_domain;
use crate::{HttpResponse, DOMAINS_DB};
use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::header::HOST;
use hyper::{Request, Response};
use std::io::Error;
use std::sync::Arc;

pub fn get_host(req: &Request<Incoming>) -> Result<&str, HttpResponse> {
    match req.uri().host().clone() {
        Some(t) => Ok(t),
        None => match req.headers().get(HOST) {
            Some(t) => match t.to_str() {
                Ok(t) => {
                    if !is_domain(t) {
                        return Err(direct_ip_reject());
                    } else {
                        Ok(t)
                    }
                }
                Err(_) => Err(internal_error("Couldn't unwrap host!")),
            },
            None => Err(direct_ip_reject()),
        },
    }
}

pub fn get_host_db(host: &str) -> Option<Arc<DomainContext>> {
    for i in DOMAINS_DB.iter() {
        if i.domain == host {
            return Some(i.clone());
        } else if host.contains(&i.domain) {
            for n in i.origin.settings.iter() {
                if n.key() == host {
                    return Some(i.clone());
                }
            }
        }
    }

    return None;
}
