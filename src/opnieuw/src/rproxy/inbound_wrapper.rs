use bytes::Bytes;
use std::pin::Pin;
use std::task::{Context, Poll, ready};
use actix_http::error::PayloadError;
use futures_core::Stream;
use pin_project_lite::pin_project;
use crate::utils::counter::Counter;
use std::sync::Arc;
use crate::models::domain_context::{DomainContext, InternalSettings};
// the wrapper for incoming streams

/// A boxed payload stream.
pub type BoxedPayloadStream = Pin<Box<dyn Stream<Item = Result<Bytes, PayloadError>>>>;


pin_project! {
    pub struct InboundWrapper<S = BoxedPayloadStream> {
        stream: S,
        context: Arc<DomainContext>,
        counter: Counter
    }
}

impl InboundWrapper {
    pub fn new(stream: BoxedPayloadStream, context: Arc<DomainContext>) -> Self {
        Self {
            stream,
            context,
            counter: Counter::new()
        }
    }
}

impl Stream for InboundWrapper {
    type Item = Result<Bytes, PayloadError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        match ready!(Pin::new(this.stream).poll_next(cx)) {
            Some(Ok(t)) => {
                // increment counter
                this.context.analytic.data_transferred_inbound.inc_by(t.len() as i64 / 1000);

                return if this.counter.inc_by(t.len() as i64 / 1000).get() < 10_000_000 {
                    Poll::Ready(Some(Ok(t)))
                } else {
                    Poll::Ready(None)
                }
            },
            _ => Poll::Ready(None) // stream has no more
        }
    }
}
/*

use std::pin::Pin;
use std::task::{Context, Poll};
use actix_web::body::{BodySize, MessageBody};
use awc::error::PayloadError;
use bytes::{Buf, Bytes};
use crate::utils::counter::Counter;
use futures_util::{ready};
use pin_project_lite::pin_project;
use std::sync::Arc;
use std::time::{Duration, Instant};
use awc::http::header::HeaderMap;
use crate::handler::pipeline::caching::models::{CachedObject, CacheLevel};
use crate::models::domain_context::DomainContext;
use awc::http::StatusCode;
use dashmap::DashMap;
use sled::IVec;
use crate::templates::error::internal_error;
use actix_web::web::Payload;
use futures_core::Stream;

pub type BoxedPayloadStream = Pin<Box<dyn Stream<Item = Result<Bytes, PayloadError>>>>;

pub struct InboundWrapper<B>
where
    B: Stream<Item = Result<Bytes, dyn Into<Box<dyn std::error::Error>>>> + 'static
{
    pub stream: B,
    pub size: Counter,
    pub size_limit: i32, // bytes / 1000
    pub domain: Arc<DomainContext>,
}

impl<B> InboundWrapper<B>
    where
        B: Stream<Item = Result<Bytes, dyn Into<Box<dyn std::error::Error>>>> + 'static {
    pub fn new(stream: B, domain: Arc<DomainContext>, size_limit: i32) -> Self {
        Self {
            stream,
            size: Counter::new(),
            size_limit,
            domain
        }
    }
}

impl<B> Stream for InboundWrapper<B>
    where
        B: Stream<Item = Result<Bytes, dyn Into<Box<dyn std::error::Error>>>> + 'static{
    type Item = Bytes;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) ->
                                                             Poll<Option<Self::Item>> {

        let this = self.project();

        if self.size.get() > self.size_limit { // test this functionality, we need to DROP the streams
            panic!("incoming body was too large, domain: {}", self.domain.domain)
        }

        match ready!(this.stream.poll_next(cx)) {
            Some(Ok(t)) => {
                self.size.inc_by(t.len() as i32 / 1000);
                return Poll::Ready(Some(t))
            },
            _ => Poll::Ready(None) // stream has no more
        }
    }
}

 */
