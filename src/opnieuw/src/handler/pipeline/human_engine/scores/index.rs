use crate::debug;
use crate::handler::pipeline::human_engine::models::HumanEngineMode;
use crate::models::request_context::RequestContext;

// Pipeline of scores

pub type PipelineFunction = fn(&RequestContext) -> u32;

#[rustfmt::skip]
lazy_static! {
    static ref PIPELINE: Vec<PipelineFunction> = vec![
        (|s: &RequestContext| -> u32 { s.check_tls() }),
        (|s: &RequestContext| -> u32 { s.check_headers() }),
        (|s: &RequestContext| -> u32 { s.check_user_agent() }),
        (|s: &RequestContext| -> u32 { s.check_analytics() }),
        (|s: &RequestContext| -> u32 { s.check_traffic_ip() }),
        (|s: &RequestContext| -> u32 { s.check_open_proxy() }),
        (|s: &RequestContext| -> u32 { s.check_internal_ratelimiters() }),
    ];
}

// TODO: CHECK BURST
// TODO: INTERNAL COUNTER FOR PATHS
// TOOD: GLOBAL ANALYTICS

impl RequestContext {
    pub fn get_score(&self) -> u32 {
        let mut score: u32 = 1;

        self.domain.analytic.ts_total_reqs.inc();

        // TODO: reintroduce as failsafe when smart challenge comes out
        //if self.check_domain_traffic() { // emergency failsafe as an immediate stopgate solution
        //    return 100
        //}

        debug!("starting human engine pipeline");

        for i in PIPELINE.iter() {
            let mut foo = i(&self);

            debug!("human engine pipeline: {}", foo);

            match self.domain.human_engine_settings.mode {
                HumanEngineMode::Chill => {
                    foo = (foo as f32 * 0.3) as u32;
                }
                HumanEngineMode::Standard => {}
                HumanEngineMode::LudicrousBotMitigation => {
                    foo = foo * 2;
                }
            }

            score += foo;

            if score >= 100 {
                // we're done enough already
                return score;
            }
        }

        debug!("leaving up to multipler");

        score * self.check_burst_multiplier()
    }
}
