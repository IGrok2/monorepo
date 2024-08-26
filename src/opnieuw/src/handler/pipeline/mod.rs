use crate::models::{
    pipeline_response::PipelineResponse,
    request_context::{
        PipelineData,
        RequestContext,
    },
};
use actix_web::HttpRequest;

pub mod api_engine;
pub mod bot_management;
pub mod caching;
pub mod human_engine;
pub mod request_inspection;
pub mod rules;
pub mod waf;

pub type PipelineFunction = fn(&RequestContext, &Vec<PipelineData>) -> PipelineResponse;

trait PipelineTrait {
    // in order of how they should be called
    // self is a RequestContext object

    /*
    At this point, everything has been checked against the global ratelimiter
    All /cdn-cgi/ requests have also been sent off to their handler
     */

    /* 1: Inspect the requests' headers
           - Users should be able to turn this on / off
    */
    fn request_inspection(&self) -> PipelineResponse;

    /* 2: API engine
       - NOTE: ratelimits should be MOSTLY global per ZONE.
       - Users should have a maximum of 3 ratelimiting BUCKETS (these are leaky buckets) that can be applied to paths
           1. checks if this path is handled by the api api_engine
               - if not, ends the pipeline there
           2. checks if the method is allowed (also is it a websocket and is that allowed & add this to req context)
               - if not, blocks the request with a nice page (or optionally, a page the user selects)
           3. checks if this request should be allowed by ratelimit, acquire bucket for this path and it to req context
               - if not, throws over a 429 with a page
           4. checks if this request should be verified (to check if it's a JWT verified user)
               - if not, return 401 unauthorized with nice page
           5. checks if it's a websocket connection request
               - if so,
                   1. checks the WAF (if the user has it on, escapes JS characters for example)
                   2. checks rules (does this websocket path allow binary, ping or text?)
                   3. sends it to backend
                   4. handles all proceeding communication, continuing to check if they're hitting the ratelimit for that bucket

           6. checks if this request needs to be micro-cached (quick cached or long cache)
               - this only works for GET requests
               - if so,
                   1. has the cache utility pull from Sled
    */
    fn api_engine(&self) -> PipelineResponse;

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

    /* 4: Verified bots -- users have a LIST of these
       - Checks if user would like to allow bots we consider verified
           - Checks if requester is bot or not
               - Uses default ratelimiting bucket for bots per domain for ALL bots to make sure
                   bots don't flood a zone
    */
    fn verified_bots(&self) -> PipelineResponse;

    /* 5: Human engine
       - At this point, all requesters should be human! Let's kick their ass if not.
           1. Generate threat score for this request
               - Uses HTTP headers & (current path calculations? could be really cool)
           2. Use threat score threshold to determine whether to serve SMART challenge or not
               - Fetches threat score threshold from RequestContext object

    */
    fn human_engine(&self) -> PipelineResponse;

    /* 6: Caching
       - How do we serve cache at high throughput? Through our beautiful DB, Sled!
           - Check if the user has that setting on
               - Call the caching utility which will do the following:
                   - Check if we have something in our DB for this subdomain+domain/path
                       - if so, stream (if over 5MB) that data back through
    */
    fn caching(&self) -> PipelineResponse;

    // Great job, request, and great job, proxy! This has made it through to the backend.
}

pub struct PipelineContext {
    pub req: HttpRequest,
    pub req_context: RequestContext,
}

impl PipelineTrait for PipelineContext {
    fn request_inspection(&self) -> PipelineResponse {
        todo!()
    }

    fn api_engine(&self) -> PipelineResponse {
        todo!()
    }

    fn verified_bots(&self) -> PipelineResponse {
        todo!()
    }

    fn human_engine(&self) -> PipelineResponse {
        todo!()
    }

    fn caching(&self) -> PipelineResponse {
        todo!()
    }
}
