// ignore compiler warnings for the following
#![allow(dead_code, unused_imports)]

// imports
use std::{
    convert::Infallible,
    iter,
    net::SocketAddr,
};

use crate::{
    bots::index::get_bots,
    buckets::global::GlobalRatelimitKeys,
    grpc::{
        client::get_all_domains,
        server::start_grpc,
    },
    handler::{
        index::{
            handler,
            handler_middleware,
        },
        models::ConnectionContext,
        pipeline::bot_management::models::Bots,
    },
    ip::models::{
        NewTrafficType,
        IP,
    },
    ip_data::open_proxies::get_open_proxies,
    models::{
        domain_context::DomainContext,
        regions::Region,
    },
    templates::error::internal_error,
    threading::index::start_background_tasks,
    tls::{
        models::{
            TlsFingerprint,
            TlsStreamWrapper,
        },
        resolver_model::CertResolver,
        ssl_models::ChallengeResponse,
    },
    utils::{
        baller::BallerLock,
        counter::Counter,
        debug::debug_setup,
        domains::get_tld_list,
        global_analytics::models::GlobalAnalytics,
        timeouts::admin_timeout,
    },
};
use aes::Aes128;
use aes_gcm::{
    aead::{
        consts::U12,
        generic_array::GenericArray,
        Aead,
        AeadMut,
    },
    Aes128Gcm,
    AesGcm,
    KeyInit,
};
use dashmap::DashMap;
use http_body_util::{
    combinators::BoxBody,
    Full,
};
use hyper::{
    body::Bytes,
    header::HeaderName,
    server::conn::{
        http1,
        http2,
    },
    service::service_fn,
    Request,
    Response,
};
use hyper_util::rt::{
    TokioExecutor,
    TokioIo,
};
use rustls_pemfile::{
    read_one,
    Item,
};
use std::{
    collections::HashMap,
    ffi::c_int,
    net::Ipv4Addr,
    process::Command,
    sync::{
        atomic::Ordering,
        Arc,
        RwLock,
    },
    time::Duration,
    u32::MAX,
};
use tokio::{
    net::TcpListener,
    task,
    time::{
        sleep,
        timeout,
    },
};
use tokio_rustls::{
    rustls,
    rustls::sign::CertifiedKey,
};

// mods
mod bots;
mod buckets;
mod cache_system;
mod grpc;
mod handler;
mod ip;
mod ip_data;
mod load_balancing;
mod models;
mod rproxy;
mod templates;
mod threading;
mod tls;
mod utils;

// response type
// pub type HttpResponse = Response<impl hyper::body::Body<Data=Bytes, Error=hyper::Error>>;
pub type HttpResponse = Response<BoxBody<Bytes, Infallible>>;
// our ordering for atomics
static ORDER: Ordering = Ordering::Relaxed;

#[macro_use]
extern crate lazy_static;

type BotsMap = BallerLock<HashMap<Bots, Arc<(Vec<String>, String)>>>;

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
    #[allow(clippy::all)]
    static ref BOTS: BotsMap = BallerLock::new(HashMap::new());
    // headers that should be on every egress response
    static ref EGRESS_HEADERS: Vec<(HeaderName, String)> = Vec::from([
        // TODO use env
        (hyper::header::VIA, std::env::var("NODE_NAME").unwrap()),
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Setting global ratelimits ...");
    println!("{:?}", dotenv::dotenv().ok().unwrap());
    GlobalRatelimitKeys::insert(
        GlobalRatelimitKeys::IpLookups,
        std::env::var("IP_LOOKUP_MAX")
            .unwrap()
            .parse::<i64>()
            .unwrap(),
    );

    println!("Purging cache ...");
    Command::new("rm").args(["-rf", "oligarch"]).spawn()?;

    // wait a second
    sleep(Duration::from_secs(1)).await;

    // create oligarch
    Command::new("mkdir").args(["oligarch"]).spawn()?;

    #[cfg(debug_assertions)]
    {
        println!("Debug mode enabled");
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_LOG", "clickhouse_rs=debug");
        debug_setup();
    }

    #[cfg(not(debug_assertions))]
    {
        println!("\nProduction mode enabled\n");

        println!("Fetching domains ...");
        get_all_domains().await.unwrap();

        println!("Starting sentry ...");
        let guard = sentry::init((
            "https://7ec929d0892caa145a432488dd07793f@sentry.packetware.net/4",
            sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            },
        ));

        println!("Sentry response: {}", guard.is_enabled());

        println!("Getting bots ...");
        get_bots().await;

        println!("Getting open proxies ...");
        let _ = PROXIES;

        println!("Getting TLDS ...");
        let _ = TLDS;

        println!("Starting gRPC server ...\n\n");
        tokio::task::spawn(start_grpc());
    }

    // start background tasks
    println!("Starting background tasks ...");
    task::spawn(start_background_tasks());

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));

    use socket2::{
        Domain,
        Protocol,
        Socket,
        Type,
    };
    let domain = Domain::for_address(addr);
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_reuse_address(true)?;
    socket.set_nonblocking(true)?;
    socket.bind(&addr.into())?;
    socket.listen(i32::MAX as u32 as c_int)?;

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::from_std(std::net::TcpListener::from(socket))?;

    // copilot, what is the lifetime of listener here?

    tokio::task::spawn(async move {
        println!("Starting HTTP/1 server on port 80...");
        // We start a loop to continuously accept incoming connections
        loop {
            if let Ok((stream, addr)) = listener.accept().await {
                // now that we have the stream, get the ip address
                let ip = IP::get(addr.ip().to_string().split(':').collect::<Vec<&str>>()[0]);

                // Use an adapter to access something implementing `tokio::io` traits as if they implement
                // `hyper::rt` IO traits.
                let io = TokioIo::new(stream);

                if ip.allow(NewTrafficType::Stream) {
                    // Spawn a tokio task to serve multiple connections concurrently
                    tokio::task::spawn(async move {
                        let counter = Arc::new(Counter::new());
                        // Finally, we bind the incoming connection to our `hello` service
                        let _ = http1::Builder::new()
                            .keep_alive(false)
                            //.header_read_timeout(Duration::from_secs(2))
                            .serve_connection(
                                io,
                                service_fn(move |req| {
                                    let ip = Arc::clone(&ip);
                                    let counter_clone = Arc::clone(&counter);

                                    counter_clone.inc();

                                    async move {
                                        handler_middleware(
                                            req,
                                            ConnectionContext {
                                                fingerprint: None,
                                                http2_counter: counter_clone,
                                                http2: false,
                                                https: false,
                                                ip,
                                            },
                                        )
                                        .await
                                    }
                                }),
                            )
                            .await;
                    });
                }
            }
        }
    });

    let https_addr = SocketAddr::from(([0, 0, 0, 0], 443));

    let domain = Domain::for_address(https_addr);
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_reuse_address(true)?;
    // ocket.set_nonblocking(true)?;
    socket.bind(&https_addr.into())?;
    socket.listen(i32::MAX as u32 as c_int)?;

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let https_listener = TcpListener::from_std(std::net::TcpListener::from(socket))?;

    let mut rustls_builder = tokio_rustls::rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_cert_resolver(Arc::new(CertResolver {}));

    rustls_builder.alpn_protocols =
        vec![b"h2".to_vec(), b"http/1.1".to_vec(), b"http/1.0".to_vec()];

    let rustls = Arc::new(tokio_rustls::TlsAcceptor::from(Arc::new(rustls_builder)));

    println!("Starting HTTPS server on port 443...");

    // We start a loop to continuously accept incoming connections
    loop {
        if let Ok((stream, addr)) = https_listener.accept().await {
            // now that we have the stream, get the ip address
            let ip = IP::get(addr.ip().to_string().split(':').collect::<Vec<&str>>()[0]);

            // check if the ip is allowed to make a new request
            if ip.allow(NewTrafficType::Stream) {
                let rustls = rustls.clone();

                let passed_ip = ip.clone();

                // Spawn a tokio task to serve multiple connections concurrently
                let handle = tokio::task::spawn(async move {
                    let io = TlsStreamWrapper::new(stream, passed_ip.ip.clone());

                    match rustls.accept(io).await {
                        Ok(acceptor) => {
                            let http2 = if let Some(protos) = acceptor.get_ref().1.alpn_protocol() {
                                protos.windows(2).any(|window| window == b"h2")
                            } else {
                                false
                            };

                            let fingerprint = acceptor.get_ref().0.parse_client_hello();
                            let counter = Arc::new(Counter::new());

                            match http2 {
                                true => {
                                    // HTTP2 connection
                                    // create a counter for the number of concurrent requests come as a result
                                    // of this connection

                                    let _ = http2::Builder::new(TokioExecutor::new())
                                        .max_pending_accept_reset_streams(5)
                                        .clone()
                                        .max_local_error_reset_streams(512)
                                        .max_concurrent_streams(50)
                                        .keep_alive_interval(None)
                                        //.keep_alive_timeout(Duration::from_secs(2))
                                        .max_header_list_size(8_000_000) // 8MB max
                                        .serve_connection(
                                            TokioIo::new(acceptor),
                                            service_fn(move |req| {
                                                debug!("http2 request: {:?}", req.uri());

                                                let ip = Arc::clone(&passed_ip);

                                                let counter_clone = Arc::clone(&counter);

                                                counter_clone.inc();

                                                async move {
                                                    handler_middleware(
                                                        req,
                                                        ConnectionContext {
                                                            fingerprint,
                                                            http2_counter: counter_clone,
                                                            http2: true,
                                                            https: true,
                                                            ip,
                                                        },
                                                    )
                                                    .await
                                                }
                                            }),
                                        )
                                        .await;
                                }
                                false => {
                                    // HTTP1 connection
                                    let _ = http1::Builder::new()
                                        .keep_alive(false)
                                        //.header_read_timeout(Duration::from_secs(2))
                                        .serve_connection(
                                            TokioIo::new(acceptor),
                                            service_fn(move |req| {
                                                let _fingerprint_clone = fingerprint;
                                                let ip = Arc::clone(&passed_ip);

                                                let counter_clone = Arc::clone(&counter);

                                                counter_clone.inc();

                                                async move {
                                                    handler_middleware(
                                                        req,
                                                        ConnectionContext {
                                                            fingerprint,
                                                            http2_counter: counter_clone,
                                                            http2: false,
                                                            https: true,
                                                            ip,
                                                        },
                                                    )
                                                    .await
                                                }
                                            }),
                                        )
                                        .await;
                                }
                            }
                        }
                        Err(e) => {
                            internal_error(&format!("Error accepting connection: {:?}", e));
                        }
                    }
                });

                // add the handle to the list of active handles
                ip.add_handle(handle.abort_handle());
            }
        }
    }
}

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
