use std::sync::atomic::AtomicUsize;
use std::time::Duration;

#[derive(Debug)] // so we can clone the object to put in the db
pub struct Ratelimiter {
    pub req: AtomicUsize, // doesn't need to be Arc'd because we already take the Arc of the obj
    pub timeout: Duration,
}
