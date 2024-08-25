use crate::handler::pipeline::human_engine::scores::failsafe::models::Failsafe;
use crate::utils::counter::Counter;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct HumanEngine {
    pub mode: HumanEngineMode,
    pub internal_bucket: InternalCounters,
    pub turbo_mode_enabled: bool,
    pub failsafe: Arc<Failsafe>,
}

#[derive(Clone, Debug)]
pub enum HumanEngineMode {
    Chill,
    Standard,
    LudicrousBotMitigation,
}

#[derive(Clone, Debug)]
pub struct InternalCounters {
    pub query_string_counter: Counter,
    pub paths: DashMap<String, Counter>,
}
