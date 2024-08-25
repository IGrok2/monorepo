// how much traffic the domain is currently seeing
// essentially the failsafe

use crate::models::request_context::RequestContext;
use crate::GA;

impl RequestContext {
    // check_domain_traffic is meant to work as an emergency stopgate solution before the threading are checked and the threshold comes back down
    pub fn check_domain_traffic(&self) -> bool {
        // we don't want the threshold to be too small, so the minimum is 101
        if self.domain.analytic.passed_human_engine.get()
            > (self.domain.internal_settings.expected_passed as f32 * 1.5) as i64 + 100
        {
            GA.handler.he.domain_traffic.inc();

            return true; // hit them with the challenge
        }
        false
    }
}
