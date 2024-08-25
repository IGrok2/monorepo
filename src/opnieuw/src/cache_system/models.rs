use crate::handler::pipeline::caching::models::CacheLevel;
use crate::models::domain_context::DomainContext;
use crate::templates::error::internal_error;
use crate::utils::counter::Counter;
use crate::GA;
use dashmap::DashMap;
use hyper::{HeaderMap, StatusCode};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct CacheBucket {
    pub map: DashMap<String, Arc<CachedObject>>,
    pub allocations: Arc<RwLock<HashMap<String, String>>>,
    pub total_size: Counter,
}

pub struct CacheAllocation {
    pub domain: Arc<DomainContext>,
    pub size: u32,
    pub location: String,
    pub created: Instant,
}

#[derive(Debug, Clone)]
pub struct CachedObject {
    pub domain: Arc<DomainContext>,
    pub location: String,
    pub id: String,
    pub headers: HeaderMap,
    pub level: CacheLevel,
    // pub status: Status,
    pub http_status: StatusCode,
    pub created: Instant,
    pub lives_until: Instant,
    pub size: usize,
}

impl Drop for CachedObject {
    fn drop(&mut self) {
        GA.cs.cache_object_drop.inc();

        if let Err(e) = Command::new("rm").args([self.location.clone()]).spawn() {
            GA.cs.cache_object_drop_delete_fail.inc();

            internal_error(&format!("error deleting cached object: {}", e));
        };
    }
}
