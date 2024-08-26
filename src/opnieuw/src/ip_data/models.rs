use crate::utils::counter::Counter;
use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct IPAsnCountry {
    pub starting_ip: Ipv4Addr,
    pub ending_ip: Ipv4Addr,

    pub asn: String,
    pub continent: String,
    // 2 letter code
    pub country: String, // 2 letter code
    pub hits: Counter,
}

#[derive(Debug, serde::Deserialize)]
pub struct InitData {
    // this is the data we get from the text file
    pub start_ip: String,
    pub end_ip: String,
    pub country: String,
    pub country_name: String,
    pub continent: String,
    pub continent_name: String,
    pub asn: String,
    pub as_name: String,
    pub as_domain: String,
}
