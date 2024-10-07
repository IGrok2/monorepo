// handles the websocket connection between requester and backend server
// applies firewalling to the websocket connection, such as allowing/blocking pings, text and binary
// also applies ratelimiting buckets
// right now i think we should follow the philosophy that request data shouldn't be created and passed through!

use actix::{
    Actor,
    AsyncContext,
    Handler,
    Recipient,
    StreamHandler,
};
use actix_codec::Framed;
use actix_http::{
    header::HeaderMap,
    ws::{
        CloseCode,
        CloseReason,
    },
};
use actix_web::{
    http::header::HeaderValue,
    web::Payload,
    HttpResponse,
};
use actix_web_actors::ws::{
    Frame,
    ProtocolError,
    WebsocketContext,
};
use awc::{
    error::WsClientError,
    http::Version,
    ws,
    ws::Codec,
    BoxedSocket,
    Client,
};
use dashmap::DashMap;
use futures::executor;
use futures_util::{
    stream::{
        SplitSink,
        SplitStream,
    },
    SinkExt,
};
use std::{
    ops::Deref,
    str::FromStr,
    sync::Arc,
};
use tokio::io::{
    AsyncRead,
    AsyncWrite,
};

use crate::{
    buckets::models::PublicBucket,
    debug,
    handler::pipeline::api_engine::models::WsMethods,
    models::{
        domain_context::OriginSetting,
        egress_wrapper::EgressWrapper,
        request_context::{
            PipelineData,
            RequestContext,
        },
    },
    templates::error::internal_error,
    tools::find_in_vec::find_in_vec,
    GA,
};
use futures_util::StreamExt;
use hyper::header::HeaderName;
use url::Url;

use crate::vars::vars::{
    NODE_NAME,
    VERSION,
};

pub async fn handle_websocket(
    context: RequestContext,
    origin: (Arc<OriginSetting>, (Url, Option<String>)),
    bucket: Option<Arc<PublicBucket>>,
    allowed_methods: Vec<WsMethods>,
    _data: &[PipelineData],
    stream: Payload,
) -> HttpResponse {
    debug!("HANDLE WEBSOCKET");

    let ws: &str = match origin.0.ssl {
        true => "wss",
        false => "ws",
    };

    let mut actual_origin: Url = origin.1 .0;

    actual_origin.set_scheme(ws).unwrap();

    actual_origin.set_path(context.req.path());

    let mut prepped_res = Client::builder()
        .max_http_version(Version::HTTP_11)
        .disable_redirects()
        .timeout(context.domain.internal_settings.req_timeout)
        .finish(); // set the timeout for this request;

    let _ = prepped_res.headers().insert(&mut HeaderMap::new());

    for i in context.req.headers.iter() {
        let v = i.0.to_string().to_lowercase();
        if v != "x-forwarded-for"
            || v != "server"
            || v != "via"
            || v != "x-country"
            || v != "x-continent"
            || v != "x-asn"
            || v != "host"
        {
            prepped_res
                .headers()
                .unwrap()
                .insert(i.0.clone(), i.1.clone());
        }
    }

    /*
    if origin.1.1.is_none() {
        prepped_res.headers().unwrap()
            .insert(HeaderName::from_str("X-Forwarded-For").unwrap(), HeaderValue::from_str(context.ip.as_str()).unwrap());
    }

     */

    if let Some(t) = origin.1 .1.clone() {
        prepped_res.headers().unwrap().insert(
            HeaderName::from_str("Host").unwrap(),
            HeaderValue::from_str(&t).unwrap(),
        );
    }

    prepped_res.headers().unwrap().insert(
        HeaderName::from_str("server").unwrap(),
        HeaderValue::from_str(&format!("cdn.camp (opnieuw)/{}", VERSION)).unwrap(),
    );

    prepped_res.headers().unwrap().insert(
        HeaderName::from_str("via").unwrap(),
        HeaderValue::from_str(NODE_NAME).unwrap(),
    );

    if origin.0.ip_data {
        // they have ip data enabled
        GA.rproxy.ip_data_requested.inc();

        let ip_data = context.get_ipdata();

        prepped_res.headers().unwrap().insert(
            HeaderName::from_str("X-Country").unwrap(),
            HeaderValue::from_str(&ip_data.0).unwrap(),
        );

        prepped_res.headers().unwrap().insert(
            HeaderName::from_str("X-Continent").unwrap(),
            HeaderValue::from_str(&ip_data.1).unwrap(),
        );

        prepped_res.headers().unwrap().insert(
            HeaderName::from_str("X-ASN").unwrap(),
            HeaderValue::from_str(&ip_data.2).unwrap(),
        );
    }

    let egress_wrapper = EgressWrapper::new(context);

    let connection = match prepped_res.ws(actual_origin.to_string()).connect().await {
        Ok(t) => t.1,
        Err(t) => {
            match t {
                // TODO: error matching
                WsClientError::InvalidResponseStatus(_t) => {} // backend was not ready to recieve
                WsClientError::InvalidUpgradeHeader => {}
                WsClientError::InvalidConnectionHeader(_) => {}
                WsClientError::MissingConnectionHeader => {}
                WsClientError::MissingWebSocketAcceptHeader => {}
                WsClientError::InvalidChallengeResponse(_, _) => {}
                WsClientError::Protocol(_) => {}
                WsClientError::SendRequest(_) => {}
            }
            GA.rproxy.websocket_error.inc();
            return egress_wrapper.ctx.origin_down();
        }
    }; // now we've got the connection to the backend but we have to connect this to the requester

    let ctx = Arc::new(egress_wrapper);

    let wrapper = WebSocketWrapper {
        conn: Arc::new(WebSocketConn::new(
            connection,
            ctx.clone(),
            allowed_methods,
            bucket,
            ctx.ctx.ip.clone(),
        )),
    };

    actix_web_actors::ws::start(wrapper, ctx.ctx.req.deref(), stream).unwrap_or_else(|x| {
        // TODO: better error management
        GA.rproxy.actor_start_error.inc();
        internal_error(&format!("Error: {x}"));
        ctx.ctx.origin_down()
    })
}

pub trait ConnectionIo: AsyncRead + AsyncWrite + Unpin + 'static {} // alias for the awc one

impl<WebSocketConn: AsyncRead + AsyncWrite + Unpin + 'static> ConnectionIo for WebSocketConn {}

pub struct WebSocketConn {
    pub sink: DashMap<u8, SplitSink<Framed<BoxedSocket, Codec>, awc::ws::Message>>,
    pub stream: DashMap<u8, SplitStream<Framed<BoxedSocket, Codec>>>,
    pub request_ctx: Arc<EgressWrapper>,
    pub allowed_methods: Vec<WsMethods>,
    pub bucket: Option<Arc<PublicBucket>>,
    pub ip: String,
}

impl WebSocketConn {
    // our connection to the backend server
    pub fn new(
        connection: Framed<BoxedSocket, Codec>,
        request_ctx: Arc<EgressWrapper>,
        allowed_methods: Vec<WsMethods>,
        bucket: Option<Arc<PublicBucket>>,
        ip: String,
    ) -> WebSocketConn {
        let (sink_a, stream_a) = connection.split();

        let sink = DashMap::new();
        sink.insert(0, sink_a);

        let stream = DashMap::new();
        stream.insert(0, stream_a);

        WebSocketConn {
            sink,
            stream,
            request_ctx,
            allowed_methods,
            ip,
            bucket,
        }
    }
}
pub struct Message(pub Frame);

impl actix::Message for Message {
    type Result = ();
}

impl Handler<Message> for WebSocketWrapper {
    // from the server to the client
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        // handle messages from the server and route them to the client
        if !self.conn.request_ctx.ctx.is_websocket_server_message_ok() {
            return; // it will do the error handling for us :p
        }

        match msg.0 {
            Frame::Text(t) => ctx.text(std::str::from_utf8(t.deref()).unwrap()), //
            Frame::Binary(t) => ctx.binary(t),
            Frame::Continuation(_t) => {}
            Frame::Ping(t) => ctx.ping(t.deref()),
            Frame::Pong(t) => ctx.pong(t.deref()),
            Frame::Close(t) => ctx.close(t),
        };
    }
}

async fn handle_server_messages(wrapper: WebSocketWrapper, ctx: Recipient<Message>) {
    let mut server = wrapper.conn.stream.get_mut(&0).unwrap();

    while let Some(msg) = server.next().await {
        match wrapper
            .conn
            .request_ctx
            .ctx
            .ws_timeout(ctx.send(Message(msg.unwrap())))
            .await
        {
            Ok(_) => {}
            Err(_) => GA.rproxy.websocket_error.inc(),
        };
    }
    /*
    loop {
        select! {
        Some(msg) = server.next() => { // go through each frame in the connection from the server
                ctx.send(Message(msg.unwrap())).await.unwrap(); // send the frame ot our "handle" method above
        }
    }
    }

     */
}

impl Actor for WebSocketConn {
    type Context = WebsocketContext<Self>; // REQUESTERS CONNECTION TO OUR SERVER
}

#[derive(Clone)]
struct WebSocketWrapper {
    conn: Arc<WebSocketConn>,
}

impl Actor for WebSocketWrapper {
    type Context = WebsocketContext<Self>;
    // REQUESTERS CONNECTION TO OUR SERVER

    fn started(&mut self, ctx: &mut Self::Context) {
        let new_self = self.clone();
        let req_ctx = new_self.conn.request_ctx.clone();
        let addr = ctx.address().clone();
        actix_web::rt::spawn(async move {
            let new_conn = req_ctx.clone(); // it's an arc
            let ctx = addr.clone();
            let _ = new_conn
                .ctx
                .total_request_timeout(handle_server_messages(new_self.clone(), ctx.recipient()))
                .await;
        });
    }
}

impl StreamHandler<Result<ws::Message, ProtocolError>> for WebSocketWrapper {
    // from the client to the server
    fn handle(&mut self, msg: Result<ws::Message, ProtocolError>, _ctx: &mut Self::Context) {
        // handle each incoming message from the client
        self.conn.request_ctx.ctx.domain.analytic.total.inc();

        match self.conn.bucket.clone() {
            Some(t) => {
                if !t.check_ip(&self.conn.ip) {
                    let reason = Some(CloseReason {
                        code: CloseCode::Policy,
                        description: None,
                    });

                    _ctx.close(reason.clone()); // TODO: message on close

                    let _ = executor::block_on(
                        self.conn
                            .sink
                            .get_mut(&0)
                            .unwrap()
                            .send(ws::Message::Close(reason)),
                    );

                    return;
                    // bucket will do the analytics
                }
            }
            None => (),
        };

        if !self.conn.request_ctx.ctx.is_websocket_message_ok() {
            return;
        }

        match msg {
            Ok(ws::Message::Ping(msg)) => {
                if !find_in_vec(&self.conn.allowed_methods, &WsMethods::Ping) {
                    // this method isn't allowed
                    GA.rproxy.ws_method_not_allowed.inc();

                    return;
                }

                GA.rproxy.ping.inc();

                let _ = executor::block_on(
                    self.conn
                        .sink
                        .get_mut(&0)
                        .unwrap()
                        .send(ws::Message::Ping(msg)),
                );
            }
            Ok(ws::Message::Text(text)) => {
                if !find_in_vec(&self.conn.allowed_methods, &WsMethods::Txt) {
                    // this method isn't allowed
                    GA.rproxy.ws_method_not_allowed.inc();

                    return;
                }

                GA.rproxy.text.inc();

                let _ = executor::block_on(
                    self.conn
                        .sink
                        .get_mut(&0)
                        .unwrap()
                        .send(ws::Message::Text(text.clone())),
                );
            }
            Ok(ws::Message::Binary(bin)) => {
                if !find_in_vec(&self.conn.allowed_methods, &WsMethods::Binary) {
                    // this method isn't allowed
                    GA.rproxy.ws_method_not_allowed.inc();

                    return;
                }

                GA.rproxy.binary.inc();

                let _ = executor::block_on(
                    self.conn
                        .sink
                        .get_mut(&0)
                        .unwrap()
                        .send(ws::Message::Binary(bin)),
                );
            }
            Ok(ws::Message::Close(reason)) => {
                // client sends the close call
                if !find_in_vec(&self.conn.allowed_methods, &WsMethods::Close) {
                    // this method isn't allowed
                    GA.rproxy.ws_method_not_allowed.inc();

                    return;
                }

                GA.rproxy.close.inc();

                let _ = executor::block_on(
                    self.conn
                        .sink
                        .get_mut(&0)
                        .unwrap()
                        .send(ws::Message::Close(reason.clone())),
                );

                _ctx.close(reason);
            }
            _ => (),
        };
    }
}

/*
impl StreamHandler<Result<ws::Message, ProtocolError>> for WebSocketConn {
    fn handle(&mut self, msg: Result<ws::Message, ProtocolError>, ctx: &mut Self::Context) { // handle each incoming message from the client
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                let hey = futures::executor::block_on(self.connection.send(ws::Message::Ping(msg)));

            }, // into makes no return value
            Ok(ws::Message::Text(text)) => {
                let _ = futures::executor::block_on(self.connection.send(ws::Message::Text(text.clone())));
                ctx.text(text)
            },
            Ok(ws::Message::Binary(bin)) => {
                let _ = futures::executor::block_on(self.connection.send(ws::Message::Binary(bin)));
            }, // put brackets here so we can have no return value
            Ok(ws::Message::Close(reason)) => { // client sends the close call
                let _ = futures::executor::block_on(self.connection.send(ws::Message::Close(reason.clone())));
                ctx.close(reason)
            }
            _ => (),
        };
    }
}

 */
