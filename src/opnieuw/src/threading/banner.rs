use std::io::{
    Read,
    Write,
};
// bans IPs (get murked)
use std::{
    net::UdpSocket,
    os::unix::net::UnixStream,
    sync::Arc,
};

use crate::{
    buckets::models::PublicBucket,
    debug,
    ip::models::IP,
    utils::counter::Counter,
    GA,
    IPS,
};
use std::{
    thread::sleep,
    time::{
        Duration,
        Instant,
    },
};

pub async fn ips() {
    /*
    let should_ban_ips = if get_g_analytic(Action::Request) > 10000 { // TODO: internal settings that manages this
      true
    } else {
      false
    };*/

    let should_ban_ips = true;

    let mut to_ban = Vec::new();
    let mut to_remove = Vec::new();

    for i in IPS.iter() {
        let reqs = i.points.get_and_reset();

        // some maintenance tasks
        i.remove_old_handles();
        i.remove_old_tokens();

        // now that we've done these tasks, let's see what these ips have been doing
        let tokens = i.get_handles_len();

        if should_ban_ips && reqs > 500 && tokens > 100 {
            to_ban.push(i.value().clone());
        } else if reqs == 0 && tokens == 0 {
            // no traffic for ip, so check if we can remove it
            if i.cycle.diff() > 600 {
                to_remove.push(i.key().clone())
            }
        }
    }

    GA.threading.ips_banned.inc_by(to_ban.len() as f64);
    GA.threading.ips_removed.inc_by(to_remove.len() as f64);
    // GA.threading.inserted.inc_by(to_insert.len() as f64);

    let ban_resp = ban_ips(to_ban.clone());

    sleep(Duration::from_secs(1));

    for i in to_remove.iter() {
        debug!("removing ip: {}", i);
        IPS.remove(i);
    }

    if ban_resp {
        for i in to_ban.iter() {
            IPS.remove(&i.ip);

            i.abort_handles();
        }
    }
    /*
    for i in to_insert.iter() {
      PublicBucket::insert_manually(i.0.clone(), i.1.clone())
    }*/
}

fn ban_ips(ips: Vec<Arc<IP>>) -> bool {
    debug!("banning {} ips!", ips.len());

    let mut to_ban = String::new();

    for i in ips.iter() {
        to_ban = to_ban + &format!("{}\n", i.ip);
    }

    if to_ban != *"" {
        if let Ok(mut stream) = UnixStream::connect("/tmp/banner") {
            stream.write_all(to_ban.as_bytes()).unwrap();

            let mut response = String::new();

            stream
                .set_read_timeout(Some(Duration::from_secs(1)))
                .unwrap();

            return match stream.read_to_string(&mut response) {
                Ok(_t) => true,
                Err(_e) => false,
            };
        } else {
            return false;
        }
    }

    false
}
