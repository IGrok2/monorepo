use crate::models::request_context::RequestContext;
use crate::{GA, PROXIES};

impl RequestContext {
    pub fn check_open_proxy(&self) -> u32 {
        for i in PROXIES.iter() {
            if &self.ip.socket_addr == i {
                GA.handler.he.open_proxy.inc();

                return 50;
            }
        }
        0
    }
}
