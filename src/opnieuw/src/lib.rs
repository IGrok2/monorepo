/*

#![allow(
dead_code,
unused_imports
)]

// ignore compiler warnings for the following
#![allow(
dead_code,
unused_imports
)]

// imports
use std::convert::Infallible;
use std::iter;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::{http1, http2};
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::net::TcpListener;
use crate::utils::global_analytics::models::GlobalAnalytics;
use std::sync::Arc;
use std::time::Duration;
use std::u32::MAX;
use dashmap::DashMap;
use hyper::header::HeaderName;
use tokio::time::{sleep, timeout};
use tokio_rustls::rustls;
use crate::ip::models::{IP, TrafficType};
use crate::tls::models::{TlsFingerprint, TlsStreamWrapper};
use rustls_pemfile::{Item, read_one};
use crate::handler::index::{handler, handler_middleware};
use crate::handler::models::ConnectionContext;
use crate::utils::counter::Counter;
use crate::models::domain_context::DomainContext;
use std::collections::HashMap;
use std::ffi::c_int;
use std::sync::RwLock;
use crate::utils::baller::BallerLock;
use crate::tls::ssl_models::ChallengeResponse;
use aes::{Aes128};
use aes_gcm::{AesGcm, Aes128Gcm, KeyInit};
use aes_gcm::aead::{Aead, AeadMut};
use aes_gcm::aead::consts::U12;
use aes_gcm::aead::generic_array::GenericArray;
use std::net::{Ipv4Addr};
use std::process::Command;
use std::sync::atomic::Ordering;
use futures_util::future::join;
use http_body_util::combinators::BoxBody;
use socket2::{Domain, Protocol, Socket, Type};
use tokio::task;
use crate::ip_data::open_proxies::get_open_proxies;
use crate::utils::domains::get_tld_list;
use tokio_rustls::rustls::sign::{CertifiedKey};
use crate::buckets::global::GlobalRatelimitKeys;
use crate::grpc::client::get_all_domains;
use crate::grpc::server::start_grpc;
use crate::handler::pipeline::bot_management::models::Bots;
use crate::templates::error::internal_error;
use crate::threading::index::start_background_tasks;
use crate::utils::debug::debug_setup;
use crate::bots::index::get_bots;
use crate::models::regions::Region;
use crate::tls::resolver_model::CertResolver;
use crate::utils::timeouts::admin_timeout;

// response type
// pub type HttpResponse = Response<impl hyper::body::Body<Data=Bytes, Error=hyper::Error>>;
pub type HttpResponse = Response<BoxBody<Bytes, Infallible>>;
// our ordering for atomics
static ORDER: Ordering = Ordering::Relaxed;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref GA: Arc<GlobalAnalytics> = Arc::new(GlobalAnalytics::new().unwrap());

    static ref IPS: DashMap<String, Arc<IP>> = DashMap::new();

    // because we write every 10 seconds
    // can't use &str here because that's a pointer
    static ref DOMAINS_DB: DashMap<String, Arc<DomainContext>> = DashMap::new();
    static ref NEW_DOMAINS_DB: RwLock<HashMap<String, Arc<DomainContext>>> = RwLock::new(HashMap::new());
    // global ratelimiter
    // static ref GLOBAL_RATELIMITER: RwLock<BTreeMap<String, Arc<Ratelimiter>>> = RwLock::new(BTreeMap::new());
    // The global ratelimiter was initially in a RwLock-protected BTreeMap, but after some testing,
    // I have concluded a DashMap is better for our use case.
    // This where we store all bots!
    static ref BOTS: BallerLock<HashMap<Bots, Arc<(Vec<String>, String)>>> = BallerLock::new(HashMap::new());
    // headers that should be on every egress response
    static ref EGRESS_HEADERS: Vec<(HeaderName, String)> = Vec::from([
        // TODO use env
        (hyper::header::VIA, "test".to_string()),
        (hyper::header::SERVER, "Packetware".to_string()),
        (hyper::header::HeaderName::from_bytes(b"cdn-loop").unwrap(), "pw".to_string())
    ]);

    static ref PROXIES: Arc<Vec<Ipv4Addr>> = Arc::new(get_open_proxies());

    static ref TLDS: Arc<Vec<String>> = Arc::new(get_tld_list());

    // LetsEncrypt challenge responses
    static ref CHALLENGE_RESPONSES: DashMap<String, Arc<ChallengeResponse>> = DashMap::new();

    // specific to the host
    static ref CERTS: DashMap<String, Arc<CertifiedKey>> = DashMap::new();
    // the wildcard host
    static ref WILDCARD_CERT: RwLock<Option<Arc<CertifiedKey>>> = RwLock::new(None);

    static ref BACKGROUND_CHALLENGE: RwLock<String> = RwLock::new("not set".to_string());

    static ref SMART_CHALLENGE: RwLock<String> = RwLock::new("not set".to_string());

    static ref CHALLENGE_KEYS: RwLock<AesGcm<Aes128, U12>> = RwLock::new(Aes128Gcm::new(GenericArray::from_slice(&[60,185,236,131,125,144,186,67,11,178,118,20,25,192,252,133])));

    // the region
    static ref REGION: Region = Region::get();
}

// provide access to the public API of the library
pub mod bots;
pub mod buckets;
pub mod cache_system;
pub mod grpc;
pub mod handler;
pub mod ip;
pub mod load_balancing;
pub mod ip_data;
pub mod models;
pub mod rproxy;
pub mod templates;
pub mod threading;
pub mod tls;
pub mod utils;

// macros for debug helpers
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($($input:tt)*) => {
        let __temp = format!($($input)*);

        println!("[DEBUG] [NON-RELEASE] [VULCAN-CORE] {}", __temp)
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($($input:tt)*) => {
        ()
    };
}

 */
