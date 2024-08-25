use crate::cache_system::writer::CacheWriter;
use crate::handler::pipeline::caching::models::CacheLevel;
use crate::models::egress_wrapper::EgressWrapper;
use crate::rproxy::outbound_wrapper::HeartInsertable::{Done, Yes};
use crate::templates::error::internal_error;
use crate::utils::counter::Counter;
use crate::GA;
use hyper::body::{Body, Bytes, Frame};
use pin_project_lite::pin_project;
use std::convert::Infallible;
use std::pin::Pin;
use std::task::{ready, Context, Poll};

pin_project! {
    pub struct OutboundWrapper {
        #[pin]
        // the stream we have to the backend
        pub stream: hyper::body::Incoming,
        // the amount of bytes that have passed through
        pub size: u64,
        // caching object
        pub cached_obj: Option<CacheWriter>,
        // turbo mode
        pub turbo: HeartInsertable,
        // for maintaining open connections and as a reference to domain
        pub egress: EgressWrapper,
        // pub req: EgressWrapper,
        // pub cache_level: DashMap<u8, CacheLevel>,
        // only if it's being cached
        // pub tbc_data: DashMap<u8, Vec<u8>>, // because we can't have anything mutable here
        // pub cache_key: Option<String>,
        // pub ttl: Option<Duration>,
        // pub alleged_size: u64,
        // pub heart_insertable: HeartInsertable,
        // pub encoding: bool,
        // pub cached_obj: Option<CacheWriter>,
        // the actual commit
    }
}

enum State {
    ReadyToPoll,
    Waiting,
    Done,
}

impl OutboundWrapper {
    pub fn new(
        stream: hyper::body::Incoming,
        cached_obj: Option<CacheWriter>,
        egress: EgressWrapper,
        turbo: HeartInsertable,
    ) -> OutboundWrapper {
        OutboundWrapper {
            stream,
            size: 0,
            cached_obj,
            egress,
            turbo,
        }
    }
}

impl Body for OutboundWrapper {
    type Data = hyper::body::Bytes;
    type Error = Infallible;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.project();

        // if we've exceeded the total limit
        if *this.turbo == Done {
            return Poll::Ready(None);
        }

        match ready!(this.stream.poll_frame(cx)) {
            Some(Ok(chunk)) => {
                // if there's data
                if let Some(chunk) = chunk.data_ref() {
                    if let Some(cached_obj) = this.cached_obj {
                        futures::executor::block_on(cached_obj.write(chunk));

                        this.egress
                            .ctx
                            .analytic
                            .data_transferred_outbound
                            .inc_by(chunk.len() as i64 / 8);

                        // the following code is block-free, true, but can take longer than expected
                        // thus it doesn't work well in a poll_frame
                        // set it under a pin
                        // let mut pinned_future = Box::pin(cached_obj.write(&chunk));
                        // use futures_util::Future;
                        // futures::ready!(Pin::new(&mut cached_obj.write(chunk)).poll_unpin(cx)).ok();
                    }

                    // also increment the length
                    *this.size += chunk.len() as u64;
                }

                Poll::Ready(Some(Ok(chunk)))
            }
            None => {
                // there's no data, so this is the end of the stream!
                if *this.turbo == Yes {
                    *this.turbo = Done;

                    this.egress.ctx.analytic.turbo_mode_served.inc();
                    GA.rproxy.turbo_mode_inserted.inc();

                    return Poll::Ready(Some(Ok(Frame::data(Bytes::from(
                        r##"<script src="/__pw/bg-js"></script>"##,
                    )))));
                }

                Poll::Ready(None)
            }
            Some(Err(e)) => {
                internal_error(&format!(
                    "error in poll is {:?}, domain: {}",
                    e, this.egress.ctx.domain
                ));

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
