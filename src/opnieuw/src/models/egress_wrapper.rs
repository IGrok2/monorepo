use crate::models::{
    domain_context::DomainContext,
    request_context::RequestContext,
};
use std::sync::Arc;

pub struct EgressWrapper {
    pub ctx: Arc<DomainContext>,
}

impl EgressWrapper {
    pub fn new(ctx: &RequestContext) -> Self {
        ctx.domain.origin.open_conns.inc();

        Self {
            ctx: ctx.domain.clone(),
        }
    }
}

impl Drop for EgressWrapper {
    fn drop(&mut self) {
        self.ctx.origin.open_conns.dec();
    }
}
