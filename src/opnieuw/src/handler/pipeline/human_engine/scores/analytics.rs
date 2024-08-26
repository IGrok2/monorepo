use crate::{
    models::request_context::RequestContext,
    GA,
};

impl RequestContext {
    pub fn check_analytics(&self) -> u32 {
        let mult =
        // compare the open connections to the allowed connections
            (self.domain.origin.open_conns.get() * 100 / self.domain.internal_settings.allowed_open_conns as i64)
                as u32;

        if mult >= 50 {
            GA.handler.he.open_conn_alert.inc();
        }

        mult
    }
}
