use crate::{
    buckets::global::GlobalRatelimitKeys,
    ip_data::models::IPAsnCountry,
    models::request_context::RequestContext,
};
use std::ops::Index;

use crate::{
    ip::models::IpData,
    ip_data::index::get_ip_data,
    GA,
};

use crate::templates::error::internal_error;

#[rustfmt::skip]
lazy_static! {
    static ref IP_DATA: Vec<IPAsnCountry> = get_ip_data();
}

impl RequestContext {
    // country, continent, asn
    pub fn get_ipdata(&self) -> (String, String, String) {
        GA.ipdata.requested.inc();

        if let Some(t) = self.ip.get_data() {
            GA.ipdata.cached.inc();

            return (t.country.clone(), t.continent.clone(), t.asn.clone());
        } else {
            GA.ipdata.miss.inc();

            if GlobalRatelimitKeys::IpLookups.is_allowed() {
                for x in IP_DATA.iter() {
                    if self.ip.socket_addr >= x.starting_ip && self.ip.socket_addr <= x.ending_ip {
                        self.ip.set_data(IpData {
                            country: x.country.clone(),
                            continent: x.continent.clone(),
                            asn: x.asn.clone(),
                        });

                        return (x.country.clone(), x.continent.clone(), x.asn.clone());
                    }
                }
            } else {
                GA.ipdata.ratelimited.inc();

                return (
                    "UNAVAILABLE".to_string(),
                    "UNAVAILABLE".to_string(),
                    "UNAVAILABLE".to_string(),
                );
            }
        }

        GA.ipdata.not_found.inc();

        // internal_error(&format!("couldn't find any data for IP: {}", &self.ip.to_string()));

        (
            "UNAVAILABLE".to_string(),
            "UNAVAILABLE".to_string(),
            "UNAVAILABLE".to_string(),
        )
    }
}
