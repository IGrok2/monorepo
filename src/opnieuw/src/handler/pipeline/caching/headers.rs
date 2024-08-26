// DEPRACTED - we add these backend headers directly now

use hyper::http::request::Builder;
use hyper::Response;
use crate::cache_system::models::CachedObject;
use crate::EGRESS_HEADERS;

impl CachedObject {
    pub fn add_backend_headers(&self, mut builder: Builder) -> Builder {
        for i in self.headers.iter() {
            builder = builder.header(i.0.as_str(), i.1.to_str());
        }
        for i in EGRESS_HEADERS.iter() {
            builder = builder.header(i.0.as_str(), &i.1); // this will overwrite any existing headers with this key
        }
        builder = builder.header("pw-cache", "hit");

        return builder
    }
}
