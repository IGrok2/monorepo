use crate::{
    DOMAINS_DB,
    ORDER,
};
use std::time::Instant;

pub async fn failsafe_loop() {
    loop {
        let _started = Instant::now();

        for i in DOMAINS_DB.iter() {
            if i.human_engine_settings.failsafe.enabled.load(ORDER) {}
        }
    }
}
