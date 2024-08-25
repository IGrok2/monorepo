// global ratelimits that shall never be crossed!
/*
Global ratelimits for protecting Packetware Systems
 */

use crate::GA;
use dashmap::iter::Iter;
use dashmap::DashMap;
use std::collections::hash_map::RandomState;

use crate::utils::counter::Counter;

lazy_static! {
    static ref GLOBAL_RATELIMITS: DashMap<String, GlobalRatelimitEntry> = DashMap::new();
};

#[derive(Debug)]
pub enum GlobalRatelimitKeys {
    IpLookups,
}

#[derive(Debug)]
pub struct GlobalRatelimitEntry {
    pub counter: Counter,
    pub threshold: i64,
}

impl GlobalRatelimitKeys {
    pub fn is_allowed(&self) -> bool {
        let query = GLOBAL_RATELIMITS
            .get(self.as_str())
            .unwrap_or_else(|| panic!("Global ratelimit key not found: {:?}", self.as_str()));

        if query.counter.inc().get() > query.threshold {
            GA.bucket.global.ratelimited.inc();

            return false;
        }

        GA.bucket.global.allowed.inc();

        true
    }

    pub fn get_iter<'a>(
    ) -> Iter<'a, String, GlobalRatelimitEntry, RandomState, DashMap<String, GlobalRatelimitEntry>>
    {
        GLOBAL_RATELIMITS.iter()
    }

    pub fn insert(key: GlobalRatelimitKeys, threshold: i64) {
        GA.bucket.global.new.inc();

        GLOBAL_RATELIMITS.insert(
            key.as_str().to_string(),
            GlobalRatelimitEntry {
                counter: Counter::new(),
                threshold,
            },
        );
    }

    // refresh the global ratelimit entries
    pub fn clear() {
        for entry in GLOBAL_RATELIMITS.iter() {
            // in the future, we could do some global analytics here to track performance of specific keys
            entry.counter.get_and_reset();
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            GlobalRatelimitKeys::IpLookups => "IpLookups",
        }
    }
}
