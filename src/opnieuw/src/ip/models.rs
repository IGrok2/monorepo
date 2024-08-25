use crate::tls::models::TlsFingerprint;
use crate::utils::counter::Counter;
use crate::utils::cycle::Cycle;
use std::net::Ipv4Addr;
use std::sync::RwLock;

#[derive(Debug)]
pub struct IP {
    // IP address
    pub ip: String,
    // the points the IP address has used
    pub points: Counter,
    // the cycle this IP address was made
    pub cycle: Cycle,
    // references behind lock
    pub protected: RwLock<ProtectedReferences>,
    // socket address
    pub socket_addr: Ipv4Addr,
    // IP data
    pub data: RwLock<Option<IpData>>,
}

#[derive(Debug)]
// these require mutability
pub struct ProtectedReferences {
    // tokens for challenge validation
    pub tokens: Vec<(Token, Cycle)>, // contains xxhash and cycle
    // AbortHandles from Tokio
    pub handles: Vec<tokio::task::AbortHandle>,
}

#[derive(Debug)]
// this is the token information
pub struct Token {
    // the user agent
    pub user_agent: String,
    // the fingerprint
    pub fingerprint: TlsFingerprint,
    // the points the token has used
    pub points: Counter,
}

#[derive(Debug, Clone)]
// this is the IP information
pub struct IpData {
    // asn
    pub asn: String,
    // country
    pub country: String,
    // continent
    pub continent: String,
}

pub enum TrafficType {
    // The request type is for a new stream
    NewStream,
    // The request type is for a new request
    NewRequest,
    // The request type is for a new token
    NewToken,
}
