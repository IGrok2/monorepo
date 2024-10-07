use crate::cache_system::reader::CacheReader;
use http_body_util::combinators::BoxBody;
use hyper::body::{
    Body,
    Bytes,
    Frame,
};
use std::{
    convert::Infallible,
    pin::Pin,
    task::{
        Context,
        Poll,
    },
};

pub enum BodyType {
    Box(BoxBody<Bytes, Infallible>),
    CacheReader(CacheReader),
}

impl Body for BodyType {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        match self.get_mut() {
            BodyType::Box(b) => b.poll_frame(cx),
            BodyType::CacheReader(r) => r.poll_frame(cx),
        }
    }
}
