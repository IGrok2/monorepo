use crate::{
    ip::models::IP,
    tls::models::TlsFingerprint,
    utils::counter::Counter,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ConnectionContext {
    pub fingerprint: Option<TlsFingerprint>,
    pub https: bool,
    pub http2: bool,
    pub http2_counter: Arc<Counter>,
    pub ip: Arc<IP>,
}
