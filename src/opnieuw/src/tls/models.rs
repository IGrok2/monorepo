use std::io::{Error, ErrorKind, IoSlice, Read};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use pin_project_lite::pin_project;
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, ReadBuf};
use tokio::net::TcpStream;
use tokio_rustls::rustls::internal::msgs::codec::Reader;
use tokio_rustls::rustls::internal::msgs::handshake::ClientExtension;
use tokio_rustls::rustls::internal::msgs::handshake::{
    ClientHelloPayload, HandshakeMessagePayload, HandshakePayload,
};
use tokio_rustls::rustls::internal::msgs::message::{Message, MessagePayload, OpaqueMessage};
use url::quirks::hash;
/*
use crate::buckets::models::PublicBucket;
use crate::{debug, GA};

 */

use crate::{debug, GA};

pin_project! {
    pub struct TlsStreamWrapper {
        #[pin]
        pub inner: TcpStream,

        pub buf: Vec<u8>,
        pub client_hello: Option<ClientHelloPayload>,
        pub ip: String
    }
}

// cancel streams by holding a reference to them, then spoof the responses and don't actually write any bytes!

impl TlsStreamWrapper {
    pub fn new(inner: TcpStream, ip: String) -> Self {
        Self {
            inner,
            buf: Vec::new(),
            client_hello: None,
            ip,
        }
    }

    pub fn client_hello(&self) -> Option<&ClientHelloPayload> {
        self.client_hello.as_ref() // pointer to the internal value
    }

    pub fn parse_client_hello(&self) -> Option<TlsFingerprint> {
        if let Some(t) = &self.client_hello {
            return Some(calculate_hexa(t));
        }

        None
    }
}

pub async fn async_read(wrapper: Arc<Pin<&mut TcpStream>>) -> std::io::Result<Vec<u8>> {
    let mut raw_buf = [0; 4096];

    let _ = Pin::new(Arc::try_unwrap(wrapper).unwrap())
        .read(&mut raw_buf)
        .await;

    Ok(raw_buf.to_vec())
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Copy)]
pub enum TlsFingerprint {
    Chrome,
    Safari,
    Firefox,
    Unknown,
}

impl AsyncRead for TlsStreamWrapper {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        // let poll = me.async_read(buf);
        let me = self.project();
        let len = buf.filled().len();
        let poll = me.inner.poll_read(cx, buf);

        if me.client_hello.is_none() {
            // the client hello will come as we accept the stream, before the request
            me.buf.extend(&buf.filled()[len..]);
            *me.client_hello = parse_client_hello(me.buf); // now set the clienthello message because it's at the beginning of the chain

            /* Not needed anymore!
            if let Some(t) = &me.client_hello {
                // PublicBucket::set_tls_fingerprint(me.ip.clone(),calculate_hexa(t));
            }

             */
        }

        poll
    }
}

use xxhash_rust::xxh3::xxh3_64;

fn calculate_hexa(payload: &ClientHelloPayload) -> TlsFingerprint {
    let mut num: String = "".to_string();

    for i in payload.cipher_suites.iter() {
        if let Some(t) = i.as_str() {
            num += t;
        }
    }

    /* this isn't very useful right now
       for i in payload.compression_methods.iter() {
           num += &i.get_u8().to_string();
       }

       for i in payload.extensions.iter() {
           match i {
               ClientExtension::ECPointFormats(_) => {}
               ClientExtension::NamedGroups(_) => {}
               ClientExtension::SignatureAlgorithms(_) => {}
               ClientExtension::ServerName(_) => {}
               ClientExtension::SessionTicket(_) => {}
               ClientExtension::Protocols(_) => {}
               ClientExtension::SupportedVersions(_) => {}
               ClientExtension::KeyShare(_) => {}
               ClientExtension::PresharedKeyModes(_) => {}
               ClientExtension::PresharedKey(_) => {}
               ClientExtension::Cookie(_) => {}
               ClientExtension::ExtendedMasterSecretRequest => {}
               ClientExtension::CertificateStatusRequest(_) => {}
               ClientExtension::SignedCertificateTimestampRequest => {}
               ClientExtension::TransportParameters(_) => {}
               ClientExtension::TransportParametersDraft(_) => {}
               ClientExtension::EarlyData => {}
               ClientExtension::Unknown(_) => {}
           }
       }

    */

    let hashed_fingerprint = xxh3_64(num.as_bytes());

    debug!("hashed fingerprint: {}", hashed_fingerprint);

    

    match hashed_fingerprint {
        1725583512279762209 | 10593271021972699412 | 8926745370823963578 => {
            GA.tls.firefox.inc();
            TlsFingerprint::Firefox
        }
        8912688367397108586 => {
            GA.tls.chrome.inc();
            TlsFingerprint::Chrome
        }
        2721214500698240047 => {
            GA.tls.safari.inc();
            TlsFingerprint::Safari
        }
        _ => {
            GA.tls.unknown.inc();
            TlsFingerprint::Unknown
        }
    }
}

fn parse_client_hello(data: &[u8]) -> Option<ClientHelloPayload> {
    let mut reader = Reader::init(data); // read the array of bytes

    // get the "opaque" message
    let msg = OpaqueMessage::read(&mut reader).ok()?; // ? to remove from Option, will return None if it doesn't work

    // get the structured message
    let msg = Message::try_from(msg.into_plain_message()).ok()?;

    if let MessagePayload::Handshake {
        parsed:
            HandshakeMessagePayload {
                // get inner payload
                payload: HandshakePayload::ClientHello(payload),
                ..
            },
        ..
    } = msg.payload
    {
        // get the inner ClientHelloPayload from our formatted message
        Some(payload)
    } else {
        None
    }
}

// now, some basics that we're forced to do to mirror the Wrapper for writing back to the network poll
impl AsyncWrite for TlsStreamWrapper {
    #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        self.project().inner.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        self.project().inner.poll_shutdown(cx)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize, Error>> {
        self.project().inner.poll_write_vectored(cx, bufs)
    }
}
