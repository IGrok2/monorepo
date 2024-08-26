use std::collections::HashMap;

use crate::{
    buckets::private::PrivateKeys::{
        AllowedWsMessages,
        CacheAttempted,
        ExpectedPassed,
        ServerMessages,
    },
    handler::pipeline::bot_management::models::Bots,
    models::{
        domain_context::InternalSettings,
        request_context::RequestContext,
    },
    utils::counter::Counter,
    GA,
};
use rapidsync::RapidSnap;
use std::{
    fmt::{
        Display,
        Formatter,
    },
    sync::RwLock,
};

// in a private bucket, there can be dynamic thresholds

#[derive(Debug)]
pub struct PrivateBucket {
    pub map: RwLock<HashMap<PrivateKeys, PrivateBucketInsert>>,
}

#[derive(PartialEq, Hash, Eq, Debug)]
pub enum PrivateKeys {
    ExpectedPassed,
    ServerMessages,
    AllowedWsMessages,
    CacheAttempted,
    AllBots, // TODO implement
    BotKey(Bots),
}

impl Display for PrivateKeys {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrivateKeys::ExpectedPassed => write!(f, "ExpectedPassed"),
            PrivateKeys::ServerMessages => write!(f, "ServerMessages"),
            PrivateKeys::AllowedWsMessages => write!(f, "AllowedWsMessages"),
            PrivateKeys::CacheAttempted => write!(f, "CacheAttempted"),
            PrivateKeys::AllBots => write!(f, "AllBots"),
            PrivateKeys::BotKey(b) => write!(f, "BotKey: {:?}", b),
        }
    }
}

#[derive(Debug)]
pub struct PrivateBucketInsert {
    pub count: Counter,
    pub threshold: u32,
}

impl PrivateBucket {
    pub fn new() -> Self {
        GA.bucket.private.new.inc();

        PrivateBucket {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub fn setup_defaults(self, is: &InternalSettings) -> Self {
        self.new_entry(ExpectedPassed, is.expected_passed as u32);
        self.new_entry(ServerMessages, is.server_messages_threshold as u32);
        self.new_entry(AllowedWsMessages, is.allowed_websocket_messages as u32);
        self.new_entry(CacheAttempted, is.attempted_cache as u32);

        self
    }

    pub fn is_allowed(&self, key: PrivateKeys) -> bool {
        let lock = self.map.read().unwrap();

        let query = lock.get(&key).unwrap(); // unwrapping is OK here because it's absolutely expected to exist

        if query.count.inc().get() > query.threshold as i64 {
            GA.bucket.private.ratelimited.inc();

            return false;
        }

        GA.bucket.private.new.inc();
        true
    }

    fn new_entry(&self, name: PrivateKeys, threshold: u32) {
        self.map.write().unwrap().insert(
            name,
            PrivateBucketInsert {
                count: Counter::new(),
                threshold,
            },
        );
    }
}

impl RequestContext {
    pub fn is_websocket_server_message_ok(&self) -> bool {
        // TODO: fix these limits. PrivateKeys cannot be used as a key because DashMap freaks out.
        // self.domain.private_bucket.is_allowed(ServerMessages)
        true
    }

    pub fn is_websocket_message_ok(&self) -> bool {
        // self.domain.private_bucket.is_allowed(AllowedWsMessages)
        true
    }
}
