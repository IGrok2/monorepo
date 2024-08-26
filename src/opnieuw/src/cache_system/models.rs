use crate::{
    handler::pipeline::caching::models::CacheLevel,
    models::domain_context::DomainContext,
    templates::error::internal_error,
    utils::counter::Counter,
    GA,
};
use dashmap::DashMap;
use hyper::{
    HeaderMap,
    StatusCode,
};
use std::{
    collections::HashMap,
    fs::File,
    path::Path,
    process::Command,
    sync::Arc,
    time::Instant,
};
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
