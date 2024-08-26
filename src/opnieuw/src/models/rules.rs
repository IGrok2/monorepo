/// DEPRECATED

use serde::{Deserialize, Serialize};

pub struct ApiEngineRule {
    // if it matches the path
    pub path: (bool, String),
    // or, if it uses the header -- if both are here then we'll check both
    // first string is the header name, second header is the optional value
    pub header: (bool, String, String),
    // then, check the rule
    pub rule: (), // the rule that should be applied at this path
        // ALLOWED method
        // ALLOWED headers
        // ALLOWED IP

    // if it hits
    pub match_action: Action,
    // typically block
    pub default_action: Action,
}

pub struct PageRule {

    pub action: Action,
}

// default deny
pub enum Action {
    Block, // block the connection based on the criteria
    Ratelimit { bucket_name: String }, // ratelimit this connection, using the specified bucket_name
    Cache { }, // TODO: send directly to the cache, let it do the work there ... this should have some data soon
    Bypass, // bypass all security checks and send directly to backend
}

/* I think, right now, these should be built ontop of ApiEngineRule
pub struct RLBucketRule { // where the hell

}

pub enum RLBucketApplicable {

}
 */
