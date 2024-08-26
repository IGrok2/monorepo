#![feature(test)]

extern crate test;

mod support;

use bytes::Bytes;
use http_body_util::{
    combinators::BoxBody,
    BodyExt,
    Full,
};
use std::{
    convert::Infallible,
    ffi::c_int,
    io::{
        Read,
        Write,
    },
    net::{
        SocketAddr,
        TcpStream,
    },
    sync::{
        mpsc,
        Arc,
    },
    time::Duration,
};

use tokio::sync::oneshot;

use hyper::{
    server::conn::http1,
    service::service_fn,
    Response,
};
use tokio::net::TcpListener;

#[bench]
fn core_test(b: &mut test::Bencher) {
    println!("core_test");
    let _ = pretty_env_logger::try_init();
    let (_until_tx, until_rx) = oneshot::channel::<()>();

    let addr = {
        println!("socket addr");
        let (addr_tx, addr_rx) = mpsc::channel();
        println!("tx rx");
        std::thread::spawn(move || {
            println!("thread spawn");
            let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("rt build");

            println!("spawn");
            use socket2::{
                Domain,
                Protocol,
                Socket,
                Type,
            };
            let domain = Domain::for_address(addr);
            let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP)).unwrap();
            socket.set_reuse_address(true).unwrap();
            socket.set_nonblocking(true).unwrap();
            socket.bind(&addr.into()).unwrap();
            socket.listen(u32::MAX as i32 as c_int).unwrap();

            // We create a TcpListener and bind it to 127.0.0.1:3000
            let listener = std::net::TcpListener::from(socket);
            listener.set_nonblocking(true).unwrap();

            let addr = listener.local_addr().unwrap();

            rt.spawn(async move {
                let listener = TcpListener::from_std(listener).unwrap();

                let counter = Arc::new(vulcancore::utils::counter::Counter::new());

                let mut current_requests = 0;
                let total_requests = 100;

                loop {
                    if current_requests >= total_requests {
                        break; // Stop accepting connections once the desired number of requests is reached
                    }

                    current_requests += 1;

                    println!("loop");
                    let (stream, _) = listener.accept().await.expect("accept");
                    let io = support::TokioIo::new(stream);

                    // now that we have the stream, get the ip address
                    use vulcancore::ip::models::IP;

                    let ip = IP::get(addr.ip().to_string().split(":").collect::<Vec<&str>>()[0]);

                    let counter = Arc::clone(&counter);

                    http1::Builder::new()
                        .serve_connection(
                            io,
                            service_fn(move |req| {
                                let ip = Arc::clone(&ip);
                                let counter = Arc::clone(&counter);

                                println!("new connection: {}", counter.inc().get());

                                use vulcancore::handler::{
                                    index::handler_middleware,
                                    models::ConnectionContext,
                                };

                                async move {
                                    handler_middleware(
                                        req,
                                        ConnectionContext {
                                            fingerprint: None,
                                            ip,
                                            http2: false,
                                            https: false,
                                            http2_counter: counter,
                                        },
                                    )
                                    .await
                                }
                            }),
                        )
                        .await
                        .unwrap();
                }
            });

            addr_tx.send(addr).unwrap();
            rt.block_on(until_rx).ok();
        });

        addr_rx.recv().unwrap()
    };

    let total_requests = 20;
    let mut current = 0;

    let mut tcp = TcpStream::connect(addr).unwrap();
    tcp.set_read_timeout(Some(Duration::from_secs(3))).unwrap();

    b.iter(|| {
        tcp.write_all(b"GET / HTTP/1.1\r\nHost: packetware.net\r\n\r\n")
            .unwrap();

        // read the response
        // let mut buf = vec![0; 1024];

        // println!("{:?}", tcp.read(&mut buf).unwrap());
    });
}
