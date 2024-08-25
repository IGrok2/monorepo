mod actions;
pub mod detections;
mod helpers;
pub mod index;
pub mod models;

/* 3: Rules!
   - Rules allows users to control traffic not from APIs. It acts on triggers and can perform actions.
       - Triggers - can either be a part of a string (beginning of path) or an entire one.
           Can match one triggers, two triggers, one trigger and not the other, etc:
               - IP address
               - Path
               - Query string
               - ASN
               - Country
               - Headers
       - Actions:
           - Captcha
           - SMART challenge
           - Block
           - Caching along with caching level
           - Redirections (and can even utilize parameters, such as redirect to diamondcdn.com/country/{country})
           - Utilize ratelimiting bucket
*/
