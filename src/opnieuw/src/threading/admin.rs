// administrative tasks that should be done every 10 seconds on the background thread

use crate::{
    buckets::global::GlobalRatelimitKeys,
    utils::cycle::Cycle,
};

pub async fn admin_tasks() {
    // now, complete any admin tasks

    // increment cycle
    Cycle::inc();

    // global ratelimit buckets
    GlobalRatelimitKeys::clear();
}
