use crate::cache_system::models::CachedObject;
use crate::models::analytics_by_example::AnalyticsByExample;
use crate::models::request_context::RequestContext;
use crate::rproxy::outbound_wrapper::HeartInsertable;
use crate::rproxy::outbound_wrapper::HeartInsertable::{Done, No, Yes};
use crate::{debug, GA};
use futures::io::Read;
use futures::{pin_mut, Stream, StreamExt};
use hyper::body::{Bytes, Frame};
use pin_project_lite::pin_project;
use std::cell::RefCell;
use std::convert::Infallible;
use std::fs;
use std::fs::read;
use std::io::BufReader;
use std::ops::Deref;
use std::path::Path;
use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time::{Duration, Instant};
use tokio::fs::File;
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWriteExt, BufWriter, ReadBuf};
use tokio::{select, task};
// TODO reinstate
//use crate::rproxy::outbound_wrapper::HeartInsertable;
//use crate::rproxy::outbound_wrapper::HeartInsertable::{Done, No, Yes};
//use crate::rproxy::pipeline::compression::{compress_zip, decompress_zip};
use crate::templates::error::internal_error;

impl CacheReader {
    pub fn new(object: &Arc<CachedObject>, turbo_mode: bool) -> Option<Self> {
        let std_file = match std::fs::File::open(&object.location) {
            Ok(t) => t,
            Err(e) => {
                internal_error(&format!("Failed to open file to send cache response on id {} with location {} and error {}", object.id, object.location, e));

                GA.cs.no_reader_file.inc();

                return None;
            }
        };

        GA.cs.new_reader.inc();

        let heart_insertable = match turbo_mode {
            true => Yes,
            false => No,
        };

        Some(CacheReader {
            reader: File::from_std(std_file),
            buf: [0; 8192],
            read_amount: 0,
            obj: object.clone(),
            turbo_mode,
            heart_insertable,
        })
    }
}

pin_project! {
    pub struct CacheReader {
        #[pin]
        pub reader: File,
        #[pin]
        pub buf: [u8; 8192],
        pub read_amount: usize,
        pub obj: Arc<CachedObject>,
        pub turbo_mode: bool,
        pub heart_insertable: HeartInsertable,
    }

    impl PinnedDrop for CacheReader {
        fn drop(this: Pin<&mut Self>) {
            // show the amounts of bytes read
            GA.cs.reader_drop.inc();

            this.obj.domain.analytic.data_transferred_cache.inc_by(this.read_amount as i64 / 1000);
        }
    }
}

impl hyper::body::Body for CacheReader {
    type Error = Infallible;

    type Data = hyper::body::Bytes;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        GA.cs.reader_poll.inc();

        let mut this = self.project();

        let mut read_buf = ReadBuf::new(&mut this.buf[..]);

        match this.reader.as_mut().poll_read(cx, &mut read_buf) {
            Poll::Ready(read) => read.unwrap(),
            Poll::Pending => {
                return Poll::Pending;
            }
        }

        this.obj
            .domain
            .analytic
            .data_transferred_cache
            .inc_by(read_buf.filled().len() as i64 / 8);

        let length = read_buf.filled().len();

        *this.read_amount += length;

        if length == 0 {
            if *this.heart_insertable == Yes {
                *this.heart_insertable = Done;

                this.obj.domain.analytic.turbo_mode_served.inc();
                GA.rproxy.turbo_mode_inserted.inc();

                GA.cs.reader_tm_insert.inc();

                return Poll::Ready(Some(Ok(Frame::data(Bytes::from(
                    r##"<script src="/__pw/bg-js"></script>"##,
                )))));
            }

            GA.cs.reader_poll_empty.inc();
        }

        // elsewise give it to them straight
        if read_buf.filled().is_empty() {
            return Poll::Ready(None);
        }

        Poll::Ready(Some(Ok(Frame::data(Bytes::from(
            read_buf.filled().to_vec(),
        )))))
    }
}
