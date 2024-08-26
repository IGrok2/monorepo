use crate::utils::counter::Counter;
use dashmap::DashMap;
use std::sync::atomic::AtomicBool;

#[derive(Debug)]
pub struct Failsafe {
    pub current_count: Counter,
    pub challenged: Counter,
    pub rolling_average: i32,
    pub enabled: AtomicBool,
    pub map: DashMap<u8, Vec<i32>>,
}
