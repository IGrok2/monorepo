use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct BallerLock<T> {
    inner: RwLock<T>,
}

impl<T> BallerLock<T> {
    pub fn new(inner: T) -> Self {
        BallerLock {
            inner: RwLock::new(inner),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<T> {
        match self.inner.read() {
            Ok(t) => t,
            Err(e) => panic!("Lock was poisoned! {}", e),
        }
    }

    pub fn write(&self) -> RwLockWriteGuard<T> {
        match self.inner.write() {
            Ok(t) => t,
            Err(e) => panic!("Lock was poisoned! {}", e),
        }
    }
}
