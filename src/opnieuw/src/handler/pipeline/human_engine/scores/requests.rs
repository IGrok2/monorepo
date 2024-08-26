// how many requests this IP has made to our services thus far

use crate::buckets::models::PublicBucket;
use crate::models::request_context::RequestContext;
use crate::GA;

impl RequestContext {
    pub fn check_traffic_ip(&self) -> u32 {
        let mult = (self.ip.points.get() as f64 / 5000_f64) as u32 * 100;

        if mult > 50 {
            GA.handler.he.request_alert.inc();
        }

        mult
    }
}
