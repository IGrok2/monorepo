// wrapper for the streamed response that we get from servers

use crate::{
    cache_system::writer::CacheWriter,
    debug,
    handler::pipeline::caching::models::CacheLevel,
    models::{
        domain_context::DomainContext,
        egress_wrapper::EgressWrapper,
        request_context::RequestContext,
    },
    rproxy::{
        outbound_wrapper::HeartInsertable::{
            Done,
            Yes,
        },
        pipeline::compression::compress_zip,
    },
    utils::counter::Counter,
    BACKGROUND_CHALLENGE,
    GA,
};
use actix_web::body::{
    BodySize,
    MessageBody,
};
use awc::{
    error::PayloadError,
    http::{
        header::HeaderMap,
        StatusCode,
    },
};
use bytes::{
    Buf,
    Bytes,
};
use dashmap::DashMap;
use futures_util::{
    ready,
    Stream,
};
use pin_project_lite::pin_project;
use sled::IVec;
use std::{
    pin::Pin,
    sync::Arc,
    task::{
        Context,
        Poll,
    },
    time::{
        Duration,
        Instant,
    },
};

pub type BoxedPayloadStream = Pin<Box<dyn Stream<Item = Result<Bytes, PayloadError>>>>;

pin_project! {
    pub struct OutboundWrapper<B>
    where B: MessageBody {
        #[pin]
        pub stream: B,
        pub size: Counter,
        pub req: EgressWrapper,
        pub cache_level: DashMap<u8, CacheLevel>,
        // only if it's being cached
        pub tbc_data: DashMap<u8, Vec<u8>>, // because we can't have anything mutable here
        pub cache_key: Option<String>,
        pub ttl: Option<Duration>,
        pub alleged_size: u64,
        pub heart_insertable: HeartInsertable,
        pub encoding: bool,
        pub cached_obj: Option<CacheWriter>,
        // the actual commit
    }

    /*
    impl<B: MessageBody> PinnedDrop for OutboundWrapper<B> {
        fn drop(cool: Pin<&mut Self>) {
            let this = cool.project();

            GA.rproxy.stream_ended.inc();

            if let Some(cache_key) = &this.cache_key {
                this.req.ctx.domain.not_being_cached(&cache_key)
            }

            this.req.ctx.domain.analytic.data_transferred_outbound.inc_by(((this.size.get() / 1000) * 2));

            this.cached_obj.as_mut().unwrap().finish_write();

            use std::ops::DerefMut; if let Some(ref mut t) = this.req.ctx.by_example.borrow_mut().deref_mut() {
                t.proxy.bytes_written = this.size.get() as u64 / 1000;
            }

            if *this.alleged_size == this.size.get() as u64 && this.cache_level.get(&0).unwrap().value() != &CacheLevel::Null && this.cache_level.get(&0).unwrap().value() != &CacheLevel::None // it's been cached
                 { // and the size is as alleged
                debug!("Decided to be cached: {:?}, {:?}, {:?}, {}", this.cache_level.get(&0).unwrap().value(), this.cache_level.get(&0).unwrap().value(), this.size.get(), this.alleged_size);

                this.req.ctx.domain.caching_settings.bucket.map.insert(this.cache_key.clone().unwrap(), // should never be None
                    Arc::new(CachedObject {
                    root_domain: this.req.ctx.domain.domain.clone(),
                    id: this.cache_key.clone().unwrap(),
                    size: this.size.get() as u32, // already been divided by 10
                    created_at: Instant::now(),
                    lasts: this.ttl.unwrap(),
                    status: this.status.unwrap(),
                    headers: this.headers.clone().unwrap(),
                    level: this.cache_level.get(&0).unwrap().clone(),
                }));
            } else {
                debug!("Decided to not be cached: {:?}, {:?}, {:?}, {}", this.cache_level.get(&0).unwrap().value(), this.cache_level.get(&0).unwrap().value(), this.size.get(), this.alleged_size);
            }
        }
    }

     */
}

impl<B: MessageBody> OutboundWrapper<B> {
    pub fn new(
        stream: B,
        size: Counter,
        req: EgressWrapper,
        cache_level: CacheLevel,
        cache_key: Option<String>,
        ttl: Option<Duration>,
        alleged_size: u64,
        heart_insertable_bool: bool,
        encoding: bool,
        cached_obj: Option<CacheWriter>,
    ) -> OutboundWrapper<B> {
        GA.rproxy.new_stream.inc();

        let cache_hashmap = DashMap::new();
        cache_hashmap.insert(0, cache_level);

        let data_hashmap = DashMap::new();
        data_hashmap.insert(0, Vec::new());

        let heart_insertable = match heart_insertable_bool {
            true => HeartInsertable::Yes,
            false => HeartInsertable::No,
        };

        OutboundWrapper {
            stream,
            size,
            req,
            cache_level: cache_hashmap,
            tbc_data: data_hashmap,
            cache_key,
            ttl,
            alleged_size,
            heart_insertable,
            encoding,
            cached_obj,
        }
    }
}

impl<B: MessageBody> MessageBody for OutboundWrapper<B> {
    type Error = B::Error;

    fn size(&self) -> BodySize {
        /*
        if let Some(t) = self.stream.size_hint().1 {
            return BodySize::Sized(t as u64)
        }
        BodySize::None
         */
        BodySize::Stream // this is a stream
    }

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        GA.rproxy.poll_next_requests.inc();

        if self.alleged_size > 0 {
            if self.size.get() as u64 > self.alleged_size {
                GA.rproxy.larger_response_than_expected.inc();
                // they lied man!
                self.cache_level.insert(0, CacheLevel::Null);
            }
        }

        let this = self.project();

        if *this.heart_insertable == Done {
            return Poll::Ready(None);
        }

        match ready!(this.stream.poll_next(cx)) {
            Some(Ok(chunk)) => {
                if let Some(t) = this.cached_obj {
                    t.write(chunk.chunk())
                }

                if *this.encoding {
                    let compressed = Bytes::from(compress_zip(chunk));

                    Poll::Ready(Some(Ok(compressed)))
                } else {
                    Poll::Ready(Some(Ok(chunk)))
                }
            }
            Some(Err(err)) => Poll::Ready(Some(Err(err))),
            None => {
                if *this.heart_insertable == Yes {
                    *this.heart_insertable = Done;

                    this.req.ctx.domain.analytic.turbo_mode_served.inc();
                    GA.rproxy.turbo_mode_inserted.inc();

                    return Poll::Ready(Some(Ok(Bytes::from(
                        r##"<script src="/__pw/bg-js"></script>"##,
                    ))));
                }

                Poll::Ready(None)
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum HeartInsertable {
    No,
    Yes,
    Done,
}
