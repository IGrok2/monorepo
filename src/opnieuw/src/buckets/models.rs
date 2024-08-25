use std::collections::hash_map::RandomState;
use std::time::{Duration, Instant};

use dashmap::iter::Iter;
use dashmap::DashMap;

use crate::ip::models::IP;
use crate::tls::models::TlsFingerprint;
use crate::utils::counter::Counter;
use crate::utils::cycle::Cycle;
use crate::{debug, GA};
use std::sync::Arc;

#[derive(Debug)]
pub struct Bucket {
    pub name: String,
    pub passed: Counter,
    pub ratelimited: Counter,

    pub threshold: u32,
    // should be cycle based
    // one cycle --> 10 seconds
    pub timeout: u32,
    pub id: String,
    pub check: bool,

    // map of pointers to the ips in this bucket
    pub ips: DashMap<String, BucketEntry>,
}

#[derive(Clone, Debug)]
pub struct PublicBucket {
    pub bucket: Arc<Bucket>,
}

#[derive(Clone, Debug)]
pub struct BucketEntry {
    pub bucket: Arc<IP>,
    pub reqs: Counter,
    pub created: Cycle,
}

impl PublicBucket {
    pub fn new(name: String, id: String, threshold: u32, timeout: u32, check: bool) -> Arc<Self> {
        GA.bucket.public.new.inc();

        let bucket = Arc::new(PublicBucket {
            bucket: Arc::new(Bucket {
                name,
                passed: Default::default(),
                ratelimited: Default::default(),
                threshold,
                timeout,
                id: id.clone(),
                check,
                ips: DashMap::new(),
            }),
        });

        bucket
    }

    pub fn check_ip(&self, ip: &Arc<IP>) -> bool {
        return match self.bucket.ips.get(&ip.ip) {
            Some(i) => {
                debug!("found bucket for check_ip");

                if i.created.diff() > self.bucket.timeout {
                    self.bucket.ips.remove(&ip.ip);

                    return false;
                }

                // increment the counter and check it
                return if i.reqs.inc().get() > self.bucket.threshold as i64 {
                    GA.bucket.public.ratelimited.inc();
                    self.bucket.ratelimited.inc();

                    false
                } else {
                    GA.bucket.public.passed.inc();
                    self.bucket.passed.inc();

                    true
                };
            }
            None => {
                // ip wasn't found in the bucket
                self.bucket.ips.insert(
                    ip.ip.clone(),
                    BucketEntry {
                        bucket: ip.clone(),
                        reqs: Default::default(),
                        created: Cycle::new(),
                    },
                );

                self.check_ip(ip)
            }
        };
    }

    // maintain buckets and remove old ips
    pub fn maintenance(&self) {
        let mut keys = Vec::new();

        // iterate through and make sure the ips aren't removed
        for i in self.bucket.ips.iter() {
            // check the cycle when it was inserted
            if i.created.diff() > self.bucket.timeout {
                keys.push(i.key().clone());
            }
        }

        for i in keys.iter() {
            self.bucket.ips.remove(i);
        }
    }
}
