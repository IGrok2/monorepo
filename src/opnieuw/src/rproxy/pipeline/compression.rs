use awc::ClientResponse;
use bytes::Bytes;
use crate::models::request_context::RequestContext;

impl RequestContext {
    pub fn can_compress(&self, backend_headers: &HeaderMap) -> bool {
        let mut can_compress = false;

        // check incoming request
        for (k, v) in self.req.headers.iter() {
            if k.to_string().to_uppercase() == "ACCEPT-ENCODING" {
                if let Ok(value) = v.to_str() {
                    debug!("Incoming accept-encoding: {}", value);
                    if value.contains("gzip") && value.contains("deflate") {
                        can_compress = true;
                    }
                }
            }
        }

        // didn't pass the checks
        if !can_compress {
            return false;
        }

        // check backend response
        for (k, v) in backend_headers {
                if k.to_string().to_uppercase() == "CONTENT-ENCODING" {
                if let Ok(value) = v.to_str() {
                    if value != "" {
                        can_compress = false;
                    }
                }
            }
        }

        can_compress
    }
}

use std::io::Read;
use std::io::Write;
use actix_http::header::HeaderMap;
use crate::debug;

pub fn compress_br<T: AsRef<[u8]>>(streamed: T) -> Vec<u8> {
    let mut input = brotli::CompressorWriter::new(Vec::new(), 4096 /* buffer size */,
                                                  5, 21);

    input.write_all(streamed.as_ref());

    let inner = input.into_inner();

    inner
}

pub fn compress_zip<T: AsRef<[u8]>>(streamed: T) -> Vec<u8> {
    let mut input = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());

    input.write_all(streamed.as_ref());

    let inner = input.finish().unwrap();

    inner
}

pub fn decompress_zip<T: AsRef<[u8]>>(streamed: T) -> Vec<u8> {
    let mut input = flate2::write::GzDecoder::new(Vec::new());

    input.write_all(streamed.as_ref());

    let inner = input.finish().unwrap();

    inner
}
