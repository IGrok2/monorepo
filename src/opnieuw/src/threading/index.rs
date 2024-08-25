/*
What should be done in the background of these systems?
    - Domain information (10 sec)
        - Analytics clearance and storage in Clickhouse
            - Requests
            - Buckets (clean the data there too), check by IP
            - Checking the cache allocations that domain has (seeing if it's abusing any limits), clear anything old, and for example stop it from caching if it does
            - Checking the expected requests per second, updating it if needs be
            - Checking the threat score threshold, making sure it is appropriate
    - Global ratelimiters
    - Private bucket ratelimiters
    - Grafana
    - Ban IPs that are being really abusive (only if there's a high amount of total, global requests, and to calculate this do it after the domain analytics
 */

// TODOS
// 1: Remake clickhouse model

use crate::threading::admin::admin_tasks;
use crate::{DOMAINS_DB, GA, IPS};
use std::time::{Duration, Instant};
use tokio::runtime::{Handle, Runtime};
use tokio::time;

use crate::threading::banner::ips;
use crate::threading::domain_analytics::index::do_domain_analytics;

pub async fn start_background_tasks() {
    // TODO: iterate through caching data, too
    loop {
        GA.threading.ran.inc();

        // let _time_started = Instant::now();

        tasks().await;

        // time::sleep(Duration::from_secs(10)).await; // so every loop starts exactly after 10 seconds
    }
}

async fn tasks() {
    /*
    println!("ips");
    ips().await;
    println!("do domain analytics");
    do_domain_analytics().await; // TODO clean up caching

     */

    // do ip once every second, analytics every 10 seconds
    for _i in 0..10 {
        ips().await;

        time::sleep(Duration::from_secs(1)).await;
    }

    admin_tasks().await;
    do_domain_analytics().await;
}
