use crate::{
    models::request_context::RequestContext,
    GA,
};

impl RequestContext {
    // the check_burst system is designed to protect domains from attack immediately
    // determines the amount to add to the threat score by a # of factors
    pub fn check_burst_multiplier(&self) -> u32 {
        // TODO: burst detection
        // the amount to add to the score
        let mut multiplier: u32 = 1;

        // add on how far it is over
        if self.domain.analytic.passed_human_engine.get()
            > self.domain.internal_settings.expected_passed as i64
        {
            GA.handler.he.multiplier_requests.inc();

            multiplier += (self.domain.analytic.passed_human_engine.get()
                / self.domain.internal_settings.expected_passed as i64)
                as u32
                / 2_u32;
        }

        if self.domain.analytic.turbo_mode_served.get() > 100
            && self.domain.analytic.turbo_mode_served.get()
                - self.domain.analytic.turbo_mode_completed.get()
                > 100
        {
            // eligible to be checked for turbo mode
            multiplier += (self.domain.analytic.turbo_mode_served.get()
                / (self.domain.analytic.turbo_mode_completed.get() + 100))
                as u32;
        }

        if self.domain.analytic.origin_err_reqs.get()
            > self.domain.internal_settings.expected_origin_errs as i64
        {
            GA.handler.he.multiplier_origin_errors.inc();

            multiplier += (self.domain.analytic.origin_err_reqs.get()
                / self.domain.internal_settings.expected_origin_errs as i64)
                as u32;
        }

        // done
        multiplier
    }
}
