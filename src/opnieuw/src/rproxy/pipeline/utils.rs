use hyper::body::Incoming;
use hyper::Response;

pub fn get_content_length(resp: &Response<Incoming>) -> u64 {
    for (i, v) in resp.headers() {
        if i.as_str().to_lowercase() == "content-length" {
            if let Ok(t) = v.to_str() {
                if let Ok(t) = t.parse::<u64>() {
                    return t;
                }
            }
        }
    }
    0
}
