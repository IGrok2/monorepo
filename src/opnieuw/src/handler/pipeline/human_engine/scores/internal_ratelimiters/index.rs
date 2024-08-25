use crate::models::request_context::RequestContext;
use crate::utils::counter::Counter;
use crate::{debug, GA};
use rand::prelude::*;
use std::time::SystemTime;

impl RequestContext {
    pub fn check_internal_ratelimiters(&self) -> u32 {
        let mut adder: u32 = 0;

        if let Some(t) = self
            .domain
            .human_engine_settings
            .internal_bucket
            .paths
            .get(self.req.uri.path())
        {
            adder += ((100 * t.inc().get()) / self.domain.internal_settings.expected_passed as i64)
                as u32;

            if adder >= 50 {
                GA.handler.he.internal_counter_alert.inc();
            }
        } else if self
            .domain
            .human_engine_settings
            .internal_bucket
            .paths
            .len()
            <= 20
        {
            // 5% chance
            debug!("path not found in human engine");
            if thread_rng().gen::<f32>() < 0.05 {
                self.domain
                    .human_engine_settings
                    .internal_bucket
                    .paths
                    .insert(self.req.uri.path().to_string(), Counter::new());
            }
        }

        adder

        /*
        if !self.req.query_string().is_empty() {
            if self.domain.human_engine_settings.internal_bucket.query_string_counter.inc().get() >
                self.domain.internal_settings.expected_query_string {
                GA.handler.he.internal_counter_alert.inc();

                return 50
            }
        }

         */
    }
}
