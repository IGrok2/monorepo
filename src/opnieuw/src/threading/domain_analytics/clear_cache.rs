use crate::{
    debug,
    models::domain_context::DomainContext,
};
use std::time::Instant;

impl DomainContext {
    pub fn clear_cache(&self) {
        // have a vector to keep track of what needs to be purged
        let mut to_delete = Vec::new();

        // check cache sizes
        // NOT USED
        // let mut cache_taken: u32 = 0;

        // iterate through cached entries and take out what's stale
        for i in self.caching_settings.bucket.map.iter() {
            if Instant::now() > i.lives_until {
                // it's expired
                // come back around to delete it
                debug!("expired cache");
                to_delete.push((i.key().clone(), i.value().clone()))
            } else {
                // else, add it to the size
                // cache_taken += i.size as u32;
            }
        }

        /* No longer needed -- fixed integer overflow issues

               // iterate through attempted cache
               for i in self.caching_settings.cache_in_progress {
                   cache_taken += i.1.1 as u32;
               }

               // set the cache counter
               // debug_assert!(cache_taken, self.caching_settings.cache_store.get());

               self.caching_settings.cache_store.set(cache_taken as i64);

        */

        // now we can iterate without the cached entries lock
        for v in to_delete.iter() {
            // remove it from the map
            self.caching_settings.bucket.map.remove(&v.0);
        }
    }
}
