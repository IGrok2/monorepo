/*
Cycles are for tracking time because instants can be too slow. This counter is updated in the background,
every 10 seconds.
 */

use crate::utils::counter::Counter;
use lazy_static::lazy_static;

// controller of the cycle which is updated in the background
#[derive(Debug)]
pub struct Cycle {
    pub cycle_num: u32,
}

// lazy static the cycle
lazy_static! {
    pub static ref CYCLE: Counter = Counter::new();
}

impl Cycle {
    // get the current cycle number
    pub fn new() -> Cycle {
        Cycle {
            cycle_num: CYCLE.get() as u32,
        }
    }

    // get the cycle difference, in 10 seconds
    pub fn diff(&self) -> u32 {
        CYCLE.get() as u32 - self.cycle_num
    }

    pub fn inc() {
        CYCLE.inc();
    }
}

impl Clone for Cycle {
    fn clone(&self) -> Self {
        Cycle {
            cycle_num: self.cycle_num,
        }
    }
}
