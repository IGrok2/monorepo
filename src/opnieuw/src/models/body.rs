use std::convert::Infallible;
use std::pin::Pin;
use std::task::{Context, Poll};
use http_body_util::combinators::BoxBody;
use hyper::body::{Body, Bytes, Frame};
use crate::cache_system::reader::CacheReader;

pub enum BodyType {
    Box(BoxBody<Bytes, Infallible>),
    CacheReader(CacheReader)
}

impl Body for BodyType {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        match self.get_mut() {
            BodyType::Box(b) => b.poll_frame(cx),
            BodyType::CacheReader(r) => r.poll_frame(cx)
        }
    }
}
