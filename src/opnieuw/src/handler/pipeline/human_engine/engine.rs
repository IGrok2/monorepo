use crate::models::pipeline_response::PipelineResponse;
use crate::models::request_context::{PipelineData, RequestContext};
use crate::GA;

/* 3: Human engine
    - At this point, all requesters should be human! Let's kick their ass if not.
        1. Generate threat score for this request
            - Uses HTTP headers & (current path calculations? could be really cool)
        2. Use threat score threshold to determine whether to serve SMART challenge or not
            - Fetches threat score threshold from RequestContext object
*/
// this REPLACES threat score!

impl RequestContext {
    pub fn human_engine(&self, _data: &[PipelineData]) -> PipelineResponse {
        // welcome to the human engine!
        GA.handler.he.inspected.inc();

        let score: u32 = if !self.is_ws() {
            self.get_score()
        } else {
            // TODO check better info for websockets
            (self.check_tls() as f64 * 1.5) as u32
        };

        use std::ops::DerefMut;
        if let Some(ref mut t) = self.by_example.borrow_mut().deref_mut() {
            t.human_engine.hit = true;
            t.human_engine.threat_score = score;
        }

        if score >= self.domain.internal_settings.threat_score_threshold as u32 {
            // we're at the limit, challenge
            GA.handler.he.challenged.inc();

            return self.smart_challenge_manager();
        }

        // increment the passed counter
        self.domain.analytic.passed_human_engine.inc();
        GA.handler.he.passed.inc();

        PipelineResponse::Ok(None)
    }
}
