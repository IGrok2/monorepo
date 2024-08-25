use crate::GA;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicI32, AtomicI64, Ordering};

// our ordering for atomics
static ORDER: Ordering = Ordering::Relaxed;

#[derive(Debug)]
pub struct Counter {
    counter: AtomicI64,
    initalized: bool,
}

// TODO: get cycler, total amount of inc to this counter, ignoring decreases (for open connections, for example)
impl Counter {
    pub fn new() -> Counter {
        GA.counter.new.inc();

        Counter {
            counter: AtomicI64::new(0),
            initalized: false,
        }
    }

    pub fn from(num: i64) -> Counter {
        Counter {
            counter: AtomicI64::new(num),
            initalized: true,
        }
    }

    pub fn set(&self, num: i64) -> &Counter {
        self.counter.store(num, ORDER);
        self
    }

    pub fn inc(&self) -> &Counter {
        GA.counter.inc.inc();

        self.counter.store(self.counter.load(ORDER) + 1, ORDER);
        self
    }

    pub fn inc_by(&self, num: i64) -> &Counter {
        GA.counter.inc.inc();

        self.counter.store(self.counter.load(ORDER) + num, ORDER);

        self
    }

    pub fn dec(&self) {
        GA.counter.dec.inc();
        self.counter.store(self.counter.load(ORDER) - 1, ORDER)
    }

    pub fn dec_by(&self, num: i64) {
        GA.counter.dec.inc();
        self.counter.store(
            // get the old value and subtract from it
            self.counter.load(ORDER) - num,
            ORDER,
        )
    }

    pub fn get(&self) -> i64 {
        GA.counter.get.inc();
        self.counter.load(ORDER)
    }

    pub fn get_and_reset(&self) -> i64 {
        let val = self.get();

        // g_analytic(Action::Counter(Reset));

        self.counter.store(0, ORDER);

        val
    }

    pub fn is_initalized(&self) -> bool {
        self.initalized
    }
}

impl Default for Counter {
    fn default() -> Self {
        Counter::new()
    }
}

impl Clone for Counter {
    fn clone(&self) -> Self {
        Self {
            counter: AtomicI64::from(self.counter.load(ORDER)),
            initalized: self.initalized,
        }
    }
}
