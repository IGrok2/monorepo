// by the way, small note about bots: EACH domain should have a RL bucket that's just for bots ...
// prevents abuse!
use crate::bots::providers::bing::get_bing;
use crate::bots::providers::google::get_google;
use crate::handler::pipeline::bot_management::models::Bots;
use std::sync::Arc;

// get_bots returns a Vector of a tuple containing a Vector
// where Vector.0 contains the IPs of the bot, and Vector.1 contains the user agent in use
pub async fn get_bots() {
    // the vector we will end up returning

    println!("Getting Googlebot ...");
    crate::BOTS.write().insert(
        Bots::Googlebot,
        Arc::new((get_google().await, "Googlebot".to_string())),
    );
    println!("Getting Bingbot ...");
    crate::BOTS.write().insert(
        Bots::Bingbot,
        Arc::new((
            get_bing().await,
            "(compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)".to_string(),
        )),
    );

    println!(
        "Googlebot IPs loaded: {}",
        crate::BOTS.read().get(&Bots::Googlebot).unwrap().0.len()
    ); // we want to unwrap here because we want to propogate an error if there is one
    println!(
        "Bingbot IPs loaded: {}",
        crate::BOTS.read().get(&Bots::Bingbot).unwrap().0.len()
    );
}

pub async fn uptime_robot() {} // TODO: offer different bots, too
