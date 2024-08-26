use crate::handler::pipeline::rules::models::Rule;

impl Rule {
    pub fn can_run(&self) -> bool {
        // determine if the rule has hit its max or can continue to be run
        if let Some(t) = self.max {
            // the max does exist
            if self.analytic.get() >= t as i64 {
                // over the max
                return false;
            }
        }

        true
    }
}
