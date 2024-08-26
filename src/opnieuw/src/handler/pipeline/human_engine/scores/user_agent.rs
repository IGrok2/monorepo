use crate::{
    models::request_context::RequestContext,
    GA,
};

impl RequestContext {
    pub fn check_user_agent(&self) -> u32 {
        let mut score = 0;
        if self.user_agent.contains("curl") || self.user_agent.contains("axois") {
            GA.handler.he.user_agent.inc();

            score += 69
        }
        score
    }
}
