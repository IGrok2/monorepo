use crate::GA;
use actix_server::Server;
use actix_web::{get, App, HttpServer, Responder};
use lazy_static::lazy_static;
use prometheus::core::{AtomicF64, GenericCounter};
use prometheus::{Encoder, Registry, TextEncoder};
use std::sync::Arc;

type PCounter = GenericCounter<AtomicF64>;

lazy_static! {
    static ref REGISTRY: Arc<Registry> = Arc::new(Registry::new());
}

#[get("/")]
async fn handler() -> impl Responder {
    let mut buffer: Vec<u8> = vec![];
    let encoder = TextEncoder::new();

    let metric_families = REGISTRY.gather();

    encoder.encode(&metric_families, &mut buffer).unwrap();
    format!("{}", String::from_utf8(buffer.clone()).unwrap())
}

impl GlobalAnalytics {
    pub async fn start() -> Server {
        let _ = GA;
        HttpServer::new(move || App::new().service(handler))
            .bind(("0.0.0.0", 9091))
            .unwrap()
            .run()
    }

    pub fn new() -> Result<GlobalAnalytics, Box<dyn std::error::Error>> {
        let requests =
            prometheus::register_counter!("overall_requests", "Total number of requests").unwrap();
        REGISTRY.register(Box::new(requests.clone()))?;
        let internal_errors = prometheus::register_counter!(
            "total_internal_errors",
            "Total number of internal errors"
        )
        .unwrap();
        REGISTRY.register(Box::new(internal_errors.clone()))?;

        let counter_new =
            prometheus::register_counter!("counter_new", "Total number of new counters").unwrap();
        REGISTRY.register(Box::new(counter_new.clone()))?;
        let counter_inc =
            prometheus::register_counter!("counter_inc", "Total number of counter increments")
                .unwrap();
        REGISTRY.register(Box::new(counter_inc.clone()))?;
        let counter_dec =
            prometheus::register_counter!("counter_dec", "Total number of counter decrements")
                .unwrap();
        REGISTRY.register(Box::new(counter_dec.clone()))?;
        let counter_get =
            prometheus::register_counter!("counter_get", "Total number of counter gets").unwrap();
        REGISTRY.register(Box::new(counter_get.clone()))?;
        let counter_reset =
            prometheus::register_counter!("counter_reset", "Total number of counter resets")
                .unwrap();
        REGISTRY.register(Box::new(counter_reset.clone()))?;

        let ips_banned =
            prometheus::register_counter!("thread_ips_banned", "Total number of ips banned")
                .unwrap();
        REGISTRY.register(Box::new(ips_banned.clone()))?;
        let ips_removed =
            prometheus::register_counter!("thread_ips_removed", "Total number of ips removed")
                .unwrap();
        REGISTRY.register(Box::new(ips_removed.clone()))?;
        let inserted =
            prometheus::register_counter!("thread_ips_inserted", "Total number of ips inserted")
                .unwrap();
        REGISTRY.register(Box::new(inserted.clone()))?;
        let ran =
            prometheus::register_counter!("thread_loops_ran", "Total number of loops run").unwrap();
        REGISTRY.register(Box::new(ran.clone()))?;

        // analytics
        let rows_inserted = prometheus::register_counter!(
            "analytics_rows_inserted",
            "Analytic row inserted for site"
        )
        .unwrap();
        REGISTRY.register(Box::new(rows_inserted.clone()))?;
        let rows_errored = prometheus::register_counter!(
            "analytics_rows_errored",
            "Analytic row errored when trying to insert for site"
        )
        .unwrap();
        REGISTRY.register(Box::new(rows_errored.clone()))?;
        let rows_timedout = prometheus::register_counter!(
            "analytics_rows_timedout",
            "Analytic row timed out when trying to insert"
        )
        .unwrap();
        REGISTRY.register(Box::new(rows_timedout.clone()))?;

        let chrome =
            prometheus::register_counter!("fingerprint_chrome", "Total number of chrome requests")
                .unwrap();
        REGISTRY.register(Box::new(chrome.clone()))?;
        let safari =
            prometheus::register_counter!("fingerprint_safari", "Total number of safari requests")
                .unwrap();
        REGISTRY.register(Box::new(safari.clone()))?;
        let firefox = prometheus::register_counter!(
            "fingerprint_firefox",
            "Total number of firefox requests"
        )
        .unwrap();
        REGISTRY.register(Box::new(firefox.clone()))?;
        let unknown = prometheus::register_counter!(
            "fingerprint_unknown",
            "Total number of unknown requests"
        )
        .unwrap();
        REGISTRY.register(Box::new(unknown.clone()))?;

        // cookie stuff
        let cookie_generated =
            prometheus::register_counter!("cookie_generated", "Number of cookies created").unwrap();
        REGISTRY.register(Box::new(cookie_generated.clone()))?;
        let cookie_tested =
            prometheus::register_counter!("cookie_tested", "Number of cookies tested").unwrap();
        REGISTRY.register(Box::new(cookie_tested.clone()))?;
        let cookie_expired =
            prometheus::register_counter!("cookie_expired", "Number of cookies expired").unwrap();
        REGISTRY.register(Box::new(cookie_expired.clone()))?;
        let cookie_misused =
            prometheus::register_counter!("cookie_misused", "Number of cookies misused").unwrap();
        REGISTRY.register(Box::new(cookie_misused.clone()))?;
        let cookie_failed = prometheus::register_counter!(
            "cookie_failed",
            "Number of cookies tested and just failed"
        )
        .unwrap();
        REGISTRY.register(Box::new(cookie_failed.clone()))?;
        let cookie_worked =
            prometheus::register_counter!("cookie_worked", "Number of cookies tested and worked")
                .unwrap();
        REGISTRY.register(Box::new(cookie_worked.clone()))?;

        let cert_requested =
            prometheus::register_counter!("tls_cert_requested", "Total number of cert requests")
                .unwrap();
        REGISTRY.register(Box::new(cert_requested.clone()))?;
        let cert_found =
            prometheus::register_counter!("tls_cert_found", "Total number of cert found").unwrap();
        REGISTRY.register(Box::new(cert_found.clone()))?;
        let cert_not_found =
            prometheus::register_counter!("tls_cert_not_found", "Total number of cert not found")
                .unwrap();
        REGISTRY.register(Box::new(cert_not_found.clone()))?;
        let cert_onpacketware = prometheus::register_counter!(
            "tls_cert_onpacketware",
            "Total number of cert on packetware"
        )
        .unwrap();
        REGISTRY.register(Box::new(cert_onpacketware.clone()))?;

        let new_stream =
            prometheus::register_counter!("rproxy_new_stream", "Total number of new streams")
                .unwrap();
        REGISTRY.register(Box::new(new_stream.clone()))?;
        let stream_ended =
            prometheus::register_counter!("rproxy_stream_ended", "Total number of stream ended")
                .unwrap();
        REGISTRY.register(Box::new(stream_ended.clone()))?;
        let received_traffic = prometheus::register_counter!(
            "rproxy_received_traffic",
            "Total number of received traffic"
        )
        .unwrap();
        REGISTRY.register(Box::new(received_traffic.clone()))?;
        let poll_next_requests = prometheus::register_counter!(
            "rproxy_poll_next_requests",
            "Total number of poll next requests"
        )
        .unwrap();
        REGISTRY.register(Box::new(poll_next_requests.clone()))?;
        let larger_response_than_expected = prometheus::register_counter!(
            "rproxy_larger_response_than_expected",
            "Total number of larger response than expected"
        )
        .unwrap();
        REGISTRY.register(Box::new(larger_response_than_expected.clone()))?;
        let cache_max_hit =
            prometheus::register_counter!("rproxy_cache_max_hit", "Total number of cache max hit")
                .unwrap();
        REGISTRY.register(Box::new(cache_max_hit.clone()))?;
        let origin_found =
            prometheus::register_counter!("rproxy_origin_found", "Total number of origin found")
                .unwrap();
        REGISTRY.register(Box::new(origin_found.clone()))?;
        let origin_not_found = prometheus::register_counter!(
            "rproxy_origin_not_found",
            "Total number of origin not found"
        )
        .unwrap();
        REGISTRY.register(Box::new(origin_not_found.clone()))?;
        let localhost =
            prometheus::register_counter!("rproxy_localhost", "Total number of localhost").unwrap();
        REGISTRY.register(Box::new(localhost.clone()))?;
        let rl_backend =
            prometheus::register_counter!("rproxy_rl_backend", "Total number of rl backend")
                .unwrap();
        REGISTRY.register(Box::new(rl_backend.clone()))?;
        let is_ws = prometheus::register_counter!("rproxy_is_ws", "Total number of is ws").unwrap();
        REGISTRY.register(Box::new(is_ws.clone()))?;
        let rproxy_ip_data_requested = prometheus::register_counter!(
            "rproxy_ip_data_requested",
            "Total number of ip data requested"
        )
        .unwrap();
        REGISTRY.register(Box::new(rproxy_ip_data_requested.clone()))?;
        let stream_allowed = prometheus::register_counter!(
            "rproxy_stream_allowed",
            "Total number of stream allowed"
        )
        .unwrap();
        REGISTRY.register(Box::new(stream_allowed.clone()))?;
        let stream_denied =
            prometheus::register_counter!("rproxy_stream_denied", "Total number of stream denied")
                .unwrap();
        REGISTRY.register(Box::new(stream_denied.clone()))?;
        let attempting_cache = prometheus::register_counter!(
            "rproxy_attempting_cache",
            "Total number of attempting cache"
        )
        .unwrap();
        REGISTRY.register(Box::new(attempting_cache.clone()))?;
        let sent_to_backend = prometheus::register_counter!(
            "rproxy_sent_to_backend",
            "Total number of sent to backend"
        )
        .unwrap();
        REGISTRY.register(Box::new(sent_to_backend.clone()))?;
        let checked_cache_headers = prometheus::register_counter!(
            "rproxy_checked_cache_headers",
            "Total number of checked cache headers"
        )
        .unwrap();
        REGISTRY.register(Box::new(checked_cache_headers.clone()))?;
        let websocket_error =
            prometheus::register_counter!("websocket_error", "Total number of websocket error")
                .unwrap();
        REGISTRY.register(Box::new(websocket_error.clone()))?;
        let actor_error =
            prometheus::register_counter!("ws_actor_error", "Total number of actor error").unwrap();
        REGISTRY.register(Box::new(actor_error.clone()))?;
        let ping = prometheus::register_counter!("ws_ping", "Total number of ping").unwrap();
        REGISTRY.register(Box::new(ping.clone()))?;
        let text = prometheus::register_counter!("ws_text", "Total number of text").unwrap();
        REGISTRY.register(Box::new(text.clone()))?;
        let binary = prometheus::register_counter!("ws_binary", "Total number of binary").unwrap();
        REGISTRY.register(Box::new(binary.clone()))?;
        let close = prometheus::register_counter!("ws_close", "Total number of close").unwrap();
        REGISTRY.register(Box::new(close.clone()))?;
        let ws_method_not_allowed = prometheus::register_counter!(
            "ws_method_not_allowed",
            "Total number of ws method not allowed"
        )
        .unwrap();
        REGISTRY.register(Box::new(ws_method_not_allowed.clone()))?;
        let turbo_mode_inserted =
            prometheus::register_counter!("turbo_mode_inserted", "Turbo mode inserted").unwrap();
        REGISTRY.register(Box::new(turbo_mode_inserted.clone()))?;

        let total_ip_data_requested = prometheus::register_counter!(
            "ip_data_requested",
            "Number of times ip data was requested overall"
        )
        .unwrap();
        REGISTRY.register(Box::new(total_ip_data_requested.clone()))?;
        let total_ip_data_cached = prometheus::register_counter!(
            "ip_data_cached",
            "Number of times we hit the IP data cache"
        )
        .unwrap();
        REGISTRY.register(Box::new(total_ip_data_cached.clone()))?;
        let total_ip_data_miss = prometheus::register_counter!(
            "ip_data_missed",
            "Number of times we missed the IP data cache"
        )
        .unwrap();
        REGISTRY.register(Box::new(total_ip_data_miss.clone()))?;
        let total_ip_data_ratelimited = prometheus::register_counter!(
            "ip_data_ratelimited",
            "Number of times querying ip data was internally ratelimited"
        )
        .unwrap();
        REGISTRY.register(Box::new(total_ip_data_ratelimited.clone()))?;
        let total_ip_data_notfound = prometheus::register_counter!(
            "ip_data_notfound",
            "Number of times querying ip data, but the IP wasn't found (this error is super weird)"
        )
        .unwrap();
        REGISTRY.register(Box::new(total_ip_data_notfound.clone()))?;

        let human_engine_domain_traffic = prometheus::register_counter!(
            "human_engine_domain_traffic_trigger",
            "The amount of traffic the domain was receiving caused us to filter this request"
        )
        .unwrap();
        REGISTRY.register(Box::new(human_engine_domain_traffic.clone()))?;
        let human_engine_headers = prometheus::register_counter!(
            "human_engine_headers_trigger",
            "The client's headers caused us to filter this request"
        )
        .unwrap();
        REGISTRY.register(Box::new(human_engine_headers.clone()))?;
        let human_engine_open_proxy = prometheus::register_counter!(
            "human_engine_open_proxy",
            "The client's IP address was in an open proxy list, causing us to filter this request"
        )
        .unwrap();
        REGISTRY.register(Box::new(human_engine_open_proxy.clone()))?;
        let human_engine_user_agent = prometheus::register_counter!(
            "human_engine_user_agent_trigger",
            "The client's user agent caused us to filter this request"
        )
        .unwrap();
        REGISTRY.register(Box::new(human_engine_user_agent.clone()))?;
        let human_engine_tls_fingerprint = prometheus::register_counter!(
            "human_engine_tls_fingerprint_trigger",
            "The client's TLS fingerprint caused us to filter this request"
        )
        .unwrap();
        REGISTRY.register(Box::new(human_engine_tls_fingerprint.clone()))?;

        let human_engine_no_tls_fingerprint = prometheus::register_counter!(
            "human_engine_no_tls_fingerprint_trigger",
            "The client's lack of TLS fingerprint caused us to filter this request"
        )
        .unwrap();
        REGISTRY.register(Box::new(human_engine_no_tls_fingerprint.clone()))?;
        let human_engine_open_conn_alert = prometheus::register_counter!(
            "human_engine_open_conn_alert_trigger",
            "The domain had a lot of open connections, so we filtered this request"
        )
        .unwrap();
        REGISTRY.register(Box::new(human_engine_open_conn_alert.clone()))?;
        let human_engine_request_alert = prometheus::register_counter!(
            "human_engine_request_alert_trigger",
            "IP had a significant amount of requests"
        )
        .unwrap();
        REGISTRY.register(Box::new(human_engine_request_alert.clone()))?;
        let human_engine_internal_counter_alert = prometheus::register_counter!(
            "human_engine_internal_counter_alert_trigger",
            "This request matched parameters that were internally ratelimited, so we filtered it"
        )
        .unwrap();
        REGISTRY.register(Box::new(human_engine_internal_counter_alert.clone()))?;
        let human_engine_multipler_requests = prometheus::register_counter!(
            "human_engine_multipler_requests",
            "human_engine_multipler_requests"
        )
        .unwrap();
        REGISTRY.register(Box::new(human_engine_multipler_requests.clone()))?;
        let human_engine_multipler_origin_errors = prometheus::register_counter!(
            "human_engine_multipler_origin_errors",
            "Origin errors multipled"
        )
        .unwrap();
        REGISTRY.register(Box::new(human_engine_multipler_origin_errors.clone()))?;

        // human engine aggregates
        let human_engine_total = prometheus::register_counter!(
            "human_engine_total",
            "The amount of requests the human engine inspected"
        )
        .unwrap();
        REGISTRY.register(Box::new(human_engine_total.clone()))?;

        let human_engine_passed = prometheus::register_counter!(
            "human_engine_passed",
            "The amount of requests the human engine let through"
        )
        .unwrap();
        REGISTRY.register(Box::new(human_engine_passed.clone()))?;

        let human_engine_challenged = prometheus::register_counter!(
            "human_engine_challenged",
            "The amount of requests the human engine decided to challenge"
        )
        .unwrap();
        REGISTRY.register(Box::new(human_engine_challenged.clone()))?;

        let request_inspection_domain_blocked = prometheus::register_counter!(
            "request_inspection_domain_blocked",
            "The domain was internally blocked"
        )
        .unwrap();
        REGISTRY.register(Box::new(request_inspection_domain_blocked.clone()))?;
        let request_inspection_cdn_loop = prometheus::register_counter!(
            "request_inspection_cdn_loop",
            "The request was detected as leaving and then hitting our infrastructure"
        )
        .unwrap();
        REGISTRY.register(Box::new(request_inspection_cdn_loop.clone()))?;
        let request_inspection_weird_user =
            prometheus::register_counter!("request_inspection_weird_user", "The client was weird!")
                .unwrap();
        REGISTRY.register(Box::new(request_inspection_weird_user.clone()))?;
        let request_inspection_header_not_string = prometheus::register_counter!(
            "request_inspection_header_not_string",
            "The client's header wasn't a string"
        )
        .unwrap();
        REGISTRY.register(Box::new(request_inspection_header_not_string.clone()))?;
        let request_inspection_waf = prometheus::register_counter!(
            "request_inspection_header_waf",
            "Request tripped the basic WAF"
        )
        .unwrap();
        REGISTRY.register(Box::new(request_inspection_waf.clone()))?;

        let page_rules_rules_tested =
            prometheus::register_counter!("page_rules_rules_tested", "Page rule triggers tested")
                .unwrap();
        REGISTRY.register(Box::new(page_rules_rules_tested.clone()))?;
        let page_rules_matches_tested =
            prometheus::register_counter!("page_rules_matches_tested", "Page rule match tested")
                .unwrap();
        REGISTRY.register(Box::new(page_rules_matches_tested.clone()))?;
        let page_rules_did_action =
            prometheus::register_counter!("page_rules_did_action", "Did page rule action").unwrap();
        REGISTRY.register(Box::new(page_rules_did_action.clone()))?;
        let page_rules_action_block = prometheus::register_counter!(
            "page_rules_action_block",
            "Request was blocked because page rule indicated so"
        )
        .unwrap();
        REGISTRY.register(Box::new(page_rules_action_block.clone()))?;
        let page_rules_action_challenge = prometheus::register_counter!(
            "page_rules_action_challenge",
            "Request was challenged because page rule indicated so"
        )
        .unwrap();
        REGISTRY.register(Box::new(page_rules_action_challenge.clone()))?;
        let page_rules_cache_data = prometheus::register_counter!(
            "page_rules_cache_data",
            "Page rule touched caching data"
        )
        .unwrap();
        REGISTRY.register(Box::new(page_rules_cache_data.clone()))?;
        let page_rules_redirect =
            prometheus::register_counter!("page_rules_redirect", "Page rule redirected").unwrap();
        REGISTRY.register(Box::new(page_rules_redirect.clone()))?;
        let page_rules_use_backend = prometheus::register_counter!(
            "page_rules_use_backend",
            "Page rule used a origin setting"
        )
        .unwrap();
        REGISTRY.register(Box::new(page_rules_use_backend.clone()))?;
        let page_rules_backend_not_found = prometheus::register_counter!(
            "page_rules_backend_not_found",
            "Page rule matched a backend that wasn't found (this shouldn't be happening)"
        )
        .unwrap();
        REGISTRY.register(Box::new(page_rules_backend_not_found.clone()))?;
        let page_rules_action_bucket =
            prometheus::register_counter!("page_rules_action_bucket", "Page rule used a bucket")
                .unwrap();
        REGISTRY.register(Box::new(page_rules_action_bucket.clone()))?;
        let page_rules_kv_tested = prometheus::register_counter!(
            "page_rules_kv_tested",
            "Page rule key-value pair tested"
        )
        .unwrap();
        REGISTRY.register(Box::new(page_rules_kv_tested.clone()))?;
        let page_rules_string_tested = prometheus::register_counter!(
            "page_rules_string_tested",
            "Page rule pure string tested"
        )
        .unwrap();
        REGISTRY.register(Box::new(page_rules_string_tested.clone()))?;

        let cache_found = prometheus::register_counter!("cache_found", "Cache was hit!").unwrap();
        REGISTRY.register(Box::new(cache_found.clone()))?;
        let cache_expired =
            prometheus::register_counter!("cache_expired", "Cache was expired").unwrap();
        REGISTRY.register(Box::new(cache_expired.clone()))?;
        let cache_metadata_existed_data_didnt = prometheus::register_counter!(
            "cache_metadata_existed_data_didnt",
            "Cache metadata was there, but the actual cache data wasn't"
        )
        .unwrap();
        REGISTRY.register(Box::new(cache_metadata_existed_data_didnt.clone()))?;
        let cache_hit =
            prometheus::register_counter!("cache_hit", "ok the cache was actually hit").unwrap();
        REGISTRY.register(Box::new(cache_hit.clone()))?;
        let cache_encoded =
            prometheus::register_counter!("cache_encoded", "cache_encoded").unwrap();
        REGISTRY.register(Box::new(cache_encoded.clone()))?;
        let cache_turbo_inserted =
            prometheus::register_counter!("cache_turbo_inserted", "cache_turbo_inserted").unwrap();
        REGISTRY.register(Box::new(cache_turbo_inserted.clone()))?;

        let bots_ratelimited = prometheus::register_counter!(
            "bots_ratelimited",
            "this bot was ratelimited for this zone"
        )
        .unwrap();
        REGISTRY.register(Box::new(bots_ratelimited.clone()))?;
        let bots_detected =
            prometheus::register_counter!("bots_detected", "Request came from a bot").unwrap();
        REGISTRY.register(Box::new(bots_detected.clone()))?;
        let bots_not_detected = prometheus::register_counter!(
            "bots_not_detected",
            "Hit the pipeline, but wasn't a bot"
        )
        .unwrap();
        REGISTRY.register(Box::new(bots_not_detected.clone()))?;

        let ws_no_engine = prometheus::register_counter!(
            "api_engine_ws_no_engine",
            "Was a websocket, but wasn't whitelisted through the engine"
        )
        .unwrap();
        REGISTRY.register(Box::new(ws_no_engine.clone()))?;
        let settings_checked = prometheus::register_counter!(
            "api_engine_settings_checked",
            "API setting conditions tested"
        )
        .unwrap();
        REGISTRY.register(Box::new(settings_checked.clone()))?;
        let whitelist_factors_checked = prometheus::register_counter!(
            "api_engine_whitelist_factors_checked",
            "API setting whitelist factors checked"
        )
        .unwrap();
        REGISTRY.register(Box::new(whitelist_factors_checked.clone()))?;
        let whitelist_ip_hit = prometheus::register_counter!(
            "api_engine_whitelist_ip_hit",
            "API engine whitelist IP hit"
        )
        .unwrap();
        REGISTRY.register(Box::new(whitelist_ip_hit.clone()))?;
        let whitelist_header_hit = prometheus::register_counter!(
            "api_engine_whitelist_header_hit",
            "API engine whitelist header hit"
        )
        .unwrap();
        REGISTRY.register(Box::new(whitelist_header_hit.clone()))?;
        let api_cache =
            prometheus::register_counter!("api_engine_cache", "API engine caching").unwrap();
        REGISTRY.register(Box::new(api_cache.clone()))?;
        let api_ratelimit =
            prometheus::register_counter!("api_engine_ratelimit", "API engine ratelimit ACTION")
                .unwrap();
        REGISTRY.register(Box::new(api_ratelimit.clone()))?;
        let setting_didnt_find_rule = prometheus::register_counter!(
            "api_engine_setting_didnt_find_rule",
            "API engine didn't find rule in the setting"
        )
        .unwrap();
        REGISTRY.register(Box::new(setting_didnt_find_rule.clone()))?;
        let api_ratelimited = prometheus::register_counter!(
            "api_engine_api_ratelimited",
            "API engine indicated bucket checking, and it was ratelimited"
        )
        .unwrap();
        REGISTRY.register(Box::new(api_ratelimited.clone()))?;

        let public_bucket_new =
            prometheus::register_counter!("public_bucket_new", "Public bucket new").unwrap();
        REGISTRY.register(Box::new(public_bucket_new.clone()))?;
        let public_bucket_ip_inserted =
            prometheus::register_counter!("public_bucket_ip_inserted", "Public bucket ip inserted")
                .unwrap();
        REGISTRY.register(Box::new(public_bucket_ip_inserted.clone()))?;
        let public_bucket_bucket_inserted = prometheus::register_counter!(
            "public_bucket_bucket_inserted",
            "Public bucket bucket inserted"
        )
        .unwrap();
        REGISTRY.register(Box::new(public_bucket_bucket_inserted.clone()))?;
        let public_bucket_passed =
            prometheus::register_counter!("public_bucket_passed", "Public bucket passed").unwrap();
        REGISTRY.register(Box::new(public_bucket_passed.clone()))?;
        let public_bucket_ratelimited =
            prometheus::register_counter!("public_bucket_ratelimited", "Public bucket ratelimited")
                .unwrap();
        REGISTRY.register(Box::new(public_bucket_ratelimited.clone()))?;
        let public_bucket_global_ratelimited = prometheus::register_counter!(
            "public_bucket_global_ratelimited",
            "Public bucket global ratelimited"
        )
        .unwrap();
        REGISTRY.register(Box::new(public_bucket_global_ratelimited.clone()))?;
        let public_bucket_ip_data_set =
            prometheus::register_counter!("public_bucket_ip_data_set", "Public bucket ip data set")
                .unwrap();
        REGISTRY.register(Box::new(public_bucket_ip_data_set.clone()))?;
        let public_bucket_ip_data_get =
            prometheus::register_counter!("public_bucket_ip_data_get", "Public bucket ip data get")
                .unwrap();
        REGISTRY.register(Box::new(public_bucket_ip_data_get.clone()))?;
        let public_bucket_expired =
            prometheus::register_counter!("public_bucket_expired", "Public bucket expired")
                .unwrap();
        REGISTRY.register(Box::new(public_bucket_expired.clone()))?;
        let public_bucket_bucket_entry_ratelimit = prometheus::register_counter!(
            "public_bucket_bucket_entry_ratelimit",
            "Public bucket bucket entry ratelimit"
        )
        .unwrap();
        REGISTRY.register(Box::new(public_bucket_bucket_entry_ratelimit.clone()))?;

        let global_bucket_ratelimited =
            prometheus::register_counter!("global_bucket_ratelimited", "Global bucket ratelimited")
                .unwrap();
        REGISTRY.register(Box::new(global_bucket_ratelimited.clone()))?;
        let global_bucket_allowed =
            prometheus::register_counter!("global_bucket_allowed", "Global bucket allowed")
                .unwrap();
        REGISTRY.register(Box::new(global_bucket_allowed.clone()))?;
        let global_bucket_new =
            prometheus::register_counter!("global_bucket_new", "Global bucket new").unwrap();
        REGISTRY.register(Box::new(global_bucket_new.clone()))?;

        let private_bucket_ratelimited = prometheus::register_counter!(
            "private_bucket_ratelimited",
            "Private bucket ratelimited"
        )
        .unwrap();
        REGISTRY.register(Box::new(private_bucket_ratelimited.clone()))?;
        let private_bucket_allowed =
            prometheus::register_counter!("private_bucket_allowed", "Private bucket allowed")
                .unwrap();
        REGISTRY.register(Box::new(private_bucket_allowed.clone()))?;
        let private_bucket_new =
            prometheus::register_counter!("private_bucket_new", "Private bucket new").unwrap();
        REGISTRY.register(Box::new(private_bucket_new.clone()))?;

        let grpc_success = prometheus::register_counter!("grpc_success", "GRPC success").unwrap();
        REGISTRY.register(Box::new(grpc_success.clone()))?;
        let grpc_fail = prometheus::register_counter!("grpc_fail", "GRPC fail").unwrap();
        REGISTRY.register(Box::new(grpc_fail.clone()))?;
        let grpc_update = prometheus::register_counter!("grpc_update", "GRPC update").unwrap();
        REGISTRY.register(Box::new(grpc_update.clone()))?;
        let grpc_new = prometheus::register_counter!("grpc_new", "GRPC new").unwrap();
        REGISTRY.register(Box::new(grpc_new.clone()))?;
        let grpc_delete = prometheus::register_counter!("grpc_delete", "GRPC delete").unwrap();
        REGISTRY.register(Box::new(grpc_delete.clone()))?;
        let grpc_challenge =
            prometheus::register_counter!("grpc_challenge", "GRPC challenge").unwrap();
        REGISTRY.register(Box::new(grpc_challenge.clone()))?;
        let grpc_challenge_removal =
            prometheus::register_counter!("grpc_challenge_removal", "GRPC challenge").unwrap();
        REGISTRY.register(Box::new(grpc_challenge_removal.clone()))?;
        let grpc_challenge_completed =
            prometheus::register_counter!("grpc_challenge_completed", "GRPC challenge").unwrap();
        REGISTRY.register(Box::new(grpc_challenge_completed.clone()))?;
        let grpc_clear_cache =
            prometheus::register_counter!("grpc_clear_cache", "GRPC clear cache").unwrap();
        REGISTRY.register(Box::new(grpc_clear_cache.clone()))?;
        let grpc_challenge_shuffle =
            prometheus::register_counter!("grpc_challenge_shuffle", "GRPC challenge shuffle")
                .unwrap();
        REGISTRY.register(Box::new(grpc_challenge_shuffle.clone()))?;

        let timeout = prometheus::register_counter!("timeouts", "Timeouts").unwrap();
        REGISTRY.register(Box::new(timeout.clone()))?;

        let streams_http = prometheus::register_counter!("streams_http", "Streams http").unwrap();
        REGISTRY.register(Box::new(streams_http.clone()))?;
        let streams_https =
            prometheus::register_counter!("streams_https", "Streams https").unwrap();
        REGISTRY.register(Box::new(streams_https.clone()))?;

        let template_rl_backend =
            prometheus::register_counter!("template_rl_backend", "Template rl backend").unwrap();
        REGISTRY.register(Box::new(template_rl_backend.clone()))?;
        let template_too_many_requests = prometheus::register_counter!(
            "template_too_many_requests",
            "Template too many requests"
        )
        .unwrap();
        REGISTRY.register(Box::new(template_too_many_requests.clone()))?;
        let template_smart_challenge =
            prometheus::register_counter!("template_smart_challenge", "Template smart challenge")
                .unwrap();
        REGISTRY.register(Box::new(template_smart_challenge.clone()))?;
        let template_origin_invalid =
            prometheus::register_counter!("template_origin_invalid", "Template origin invalid")
                .unwrap();
        REGISTRY.register(Box::new(template_origin_invalid.clone()))?;
        let template_origin_down =
            prometheus::register_counter!("template_origin_down", "Template origin down").unwrap();
        REGISTRY.register(Box::new(template_origin_down.clone()))?;
        let template_waf = prometheus::register_counter!("template_waf", "Template waf").unwrap();
        REGISTRY.register(Box::new(template_waf.clone()))?;
        let template_invalid =
            prometheus::register_counter!("template_invalid", "Template invalid").unwrap();
        REGISTRY.register(Box::new(template_invalid.clone()))?;
        let template_global_ratelimit =
            prometheus::register_counter!("template_global_ratelimit", "Template global ratelimit")
                .unwrap();
        REGISTRY.register(Box::new(template_global_ratelimit.clone()))?;
        let template_domain_not_found =
            prometheus::register_counter!("template_domain_not_found", "Template domain not found")
                .unwrap();
        REGISTRY.register(Box::new(template_domain_not_found.clone()))?;
        let template_direct_ip =
            prometheus::register_counter!("template_direct_ip", "Template direct ip").unwrap();
        REGISTRY.register(Box::new(template_direct_ip.clone()))?;
        let template_blocked =
            prometheus::register_counter!("template_blocked", "Template blocked").unwrap();
        REGISTRY.register(Box::new(template_blocked.clone()))?;
        let template_api_engine_blocked = prometheus::register_counter!(
            "template_api_engine_blocked",
            "Template api engine blocked"
        )
        .unwrap();
        REGISTRY.register(Box::new(template_api_engine_blocked.clone()))?;
        let template_internal_error =
            prometheus::register_counter!("template_internal_error", "Template internal error")
                .unwrap();
        REGISTRY.register(Box::new(template_internal_error.clone()))?;
        let template_api_engine_required = prometheus::register_counter!(
            "template_api_engine_required",
            "Template api engine required"
        )
        .unwrap();
        REGISTRY.register(Box::new(template_api_engine_required.clone()))?;

        let cgi_request =
            prometheus::register_counter!("cgi_request", "Request to the CGI").unwrap();
        REGISTRY.register(Box::new(cgi_request.clone()))?;
        let submit_challenge_request = prometheus::register_counter!(
            "submit_challenge_request",
            "Request to submit the challenge"
        )
        .unwrap();
        REGISTRY.register(Box::new(submit_challenge_request.clone()))?;
        let submit_challenge_success = prometheus::register_counter!(
            "submit_challenge_success",
            "Request to submit the challenge went well"
        )
        .unwrap();
        REGISTRY.register(Box::new(submit_challenge_success.clone()))?;
        let submit_challenge_fail = prometheus::register_counter!(
            "submit_challenge_fail",
            "Request to submit the challenge failed"
        )
        .unwrap();
        REGISTRY.register(Box::new(submit_challenge_fail.clone()))?;
        let submit_challenge_timeout =
            prometheus::register_counter!("submit_challenge_timeout", "Reading body timed out")
                .unwrap();
        REGISTRY.register(Box::new(submit_challenge_timeout.clone()))?;
        let submit_challenge_size =
            prometheus::register_counter!("submit_challenge_size", "Submitting challenge failed")
                .unwrap();
        REGISTRY.register(Box::new(submit_challenge_size.clone()))?;
        let challenge_not_utf8 =
            prometheus::register_counter!("challenge_not_utf8", "Submitting challenge failed")
                .unwrap();
        REGISTRY.register(Box::new(challenge_not_utf8.clone()))?;
        let challenge_bad_base64 =
            prometheus::register_counter!("challenge_bad_base64", "Submitting challenge failed")
                .unwrap();
        REGISTRY.register(Box::new(challenge_bad_base64.clone()))?;
        let challenge_too_short =
            prometheus::register_counter!("challenge_too_short", "Submitting challenge failed")
                .unwrap();
        REGISTRY.register(Box::new(challenge_too_short.clone()))?;
        let challenge_decryption_fail = prometheus::register_counter!(
            "challenge_decryption_fail",
            "Submitting challenge failed"
        )
        .unwrap();
        REGISTRY.register(Box::new(challenge_decryption_fail.clone()))?;
        let challenge_decryption_not_utf8 = prometheus::register_counter!(
            "challenge_decryption_not_utf8",
            "Submitting challenge failed"
        )
        .unwrap();
        REGISTRY.register(Box::new(challenge_decryption_not_utf8.clone()))?;
        let challenge_unable_to_format = prometheus::register_counter!(
            "challenge_unable_to_format",
            "Submitting challenge failed"
        )
        .unwrap();
        REGISTRY.register(Box::new(challenge_unable_to_format.clone()))?;
        let challenge_ua_no_match =
            prometheus::register_counter!("challenge_ua_no_match", "Submitting challenge failed")
                .unwrap();
        REGISTRY.register(Box::new(challenge_ua_no_match.clone()))?;
        let challenge_timeout_replay = prometheus::register_counter!(
            "challenge_timeout_replay",
            "Submitting challenge failed"
        )
        .unwrap();
        REGISTRY.register(Box::new(challenge_timeout_replay.clone()))?;
        let challenge_browser_no_match = prometheus::register_counter!(
            "challenge_browser_no_match",
            "Submitting challenge failed"
        )
        .unwrap();
        REGISTRY.register(Box::new(challenge_browser_no_match.clone()))?;
        let challenge_notifications_not_persistent = prometheus::register_counter!(
            "challenge_notifications_not_persistent",
            "Submitting challenge failed"
        )
        .unwrap();
        REGISTRY.register(Box::new(challenge_notifications_not_persistent.clone()))?;
        let challenge_fixed_memory_set = prometheus::register_counter!(
            "challenge_fixed_memory_set",
            "Submitting challenge failed"
        )
        .unwrap();
        REGISTRY.register(Box::new(challenge_fixed_memory_set.clone()))?;

        let challenge_audio_context =
            prometheus::register_counter!("challenge_audio_context", "Submitting challenge failed")
                .unwrap();
        REGISTRY.register(Box::new(challenge_audio_context.clone()))?;

        let challenge_chrome =
            prometheus::register_counter!("challenge_chrome", "Submitting challenge failed")
                .unwrap();
        REGISTRY.register(Box::new(challenge_chrome.clone()))?;
        let challenge_firefox =
            prometheus::register_counter!("challenge_firefox", "Submitting challenge failed")
                .unwrap();
        REGISTRY.register(Box::new(challenge_firefox.clone()))?;
        let challenge_safari =
            prometheus::register_counter!("challenge_safari", "Submitting challenge failed")
                .unwrap();
        REGISTRY.register(Box::new(challenge_safari.clone()))?;
        let challenge_unknown =
            prometheus::register_counter!("challenge_unknown", "Submitting challenge failed")
                .unwrap();
        REGISTRY.register(Box::new(challenge_unknown.clone()))?;

        let well_known =
            prometheus::register_counter!("well_known", "Requests made to the LE well-known")
                .unwrap();
        REGISTRY.register(Box::new(well_known.clone()))?;
        let well_known_token_found = prometheus::register_counter!(
            "well_known_token_found",
            "Requests made to the LE well-known"
        )
        .unwrap();
        REGISTRY.register(Box::new(well_known_token_found.clone()))?;
        let well_known_token_not_found = prometheus::register_counter!(
            "well_known_token_not_found",
            "Requests made to the LE well-known"
        )
        .unwrap();
        REGISTRY.register(Box::new(well_known_token_not_found.clone()))?;

        // caching system
        let cs_new_reader =
            prometheus::register_counter!("cs_new_reader", "Cache system debuggers").unwrap();
        REGISTRY.register(Box::new(cs_new_reader.clone()))?;
        let cs_no_reader_file =
            prometheus::register_counter!("cs_no_reader_file", "Cache system debuggers").unwrap();
        REGISTRY.register(Box::new(cs_no_reader_file.clone()))?;
        let cs_reader_drop =
            prometheus::register_counter!("cs_reader_drop", "Cache system debuggers").unwrap();
        REGISTRY.register(Box::new(cs_reader_drop.clone()))?;
        let cs_reader_poll =
            prometheus::register_counter!("cs_reader_poll", "Cache system debuggers").unwrap();
        REGISTRY.register(Box::new(cs_reader_poll.clone()))?;
        let cs_reader_poll_empty =
            prometheus::register_counter!("cs_reader_poll_empty", "Cache system debuggers")
                .unwrap();
        REGISTRY.register(Box::new(cs_reader_poll_empty.clone()))?;
        let cs_reader_poll_complete =
            prometheus::register_counter!("cs_reader_poll_complete", "Cache system debuggers")
                .unwrap();
        REGISTRY.register(Box::new(cs_reader_poll_complete.clone()))?;
        let cs_reader_tm_insert =
            prometheus::register_counter!("cs_reader_tm_insert", "Cache system debuggers").unwrap();
        REGISTRY.register(Box::new(cs_reader_tm_insert.clone()))?;
        // writer
        let cs_new_writer =
            prometheus::register_counter!("cs_new_writer", "Cache system debuggers").unwrap();
        REGISTRY.register(Box::new(cs_new_writer.clone()))?;
        let cs_writer_create_directory =
            prometheus::register_counter!("cs_writer_create_directory", "Cache system debuggers")
                .unwrap();
        REGISTRY.register(Box::new(cs_writer_create_directory.clone()))?;
        let cs_writer_create_directory_fail = prometheus::register_counter!(
            "cs_writer_create_directory_fail",
            "Cache system debuggers"
        )
        .unwrap();
        REGISTRY.register(Box::new(cs_writer_create_directory_fail.clone()))?;
        let cs_writer_write =
            prometheus::register_counter!("cs_writer_write", "Cache system debuggers").unwrap();
        REGISTRY.register(Box::new(cs_writer_write.clone()))?;
        let cs_writer_write_ok =
            prometheus::register_counter!("cs_writer_write_ok", "Cache system debuggers").unwrap();
        REGISTRY.register(Box::new(cs_writer_write_ok.clone()))?;
        let cs_writer_write_err =
            prometheus::register_counter!("cs_writer_write_err", "Cache system debuggers").unwrap();
        REGISTRY.register(Box::new(cs_writer_write_err.clone()))?;
        let cs_writer_drop =
            prometheus::register_counter!("cs_writer_drop", "Cache system debuggers").unwrap();
        REGISTRY.register(Box::new(cs_writer_drop.clone()))?;
        let cs_writer_drop_finish =
            prometheus::register_counter!("cs_writer_drop_finish", "Cache system debuggers")
                .unwrap();
        REGISTRY.register(Box::new(cs_writer_drop_finish.clone()))?;
        let cs_writer_non_expected_length = prometheus::register_counter!(
            "cs_writer_non_expected_length",
            "Cache system debuggers"
        )
        .unwrap();
        REGISTRY.register(Box::new(cs_writer_non_expected_length.clone()))?;
        let cs_writer_attempt_insert =
            prometheus::register_counter!("cs_writer_attempt_insert", "Cache system debuggers")
                .unwrap();
        REGISTRY.register(Box::new(cs_writer_attempt_insert.clone()))?;
        let cs_writer_finish_insert =
            prometheus::register_counter!("cs_writer_finish_insert", "Cache system debuggers")
                .unwrap();
        REGISTRY.register(Box::new(cs_writer_finish_insert.clone()))?;
        let cs_writer_finish_write =
            prometheus::register_counter!("cs_writer_finish_write", "Cache system debuggers")
                .unwrap();
        REGISTRY.register(Box::new(cs_writer_finish_write.clone()))?;
        let cs_writer_finish_write_finished = prometheus::register_counter!(
            "cs_writer_finish_write_finished",
            "Cache system debuggers"
        )
        .unwrap();
        REGISTRY.register(Box::new(cs_writer_finish_write_finished.clone()))?;
        // cached object
        let cs_cache_object_drop =
            prometheus::register_counter!("cs_cache_object_drop", "Cache system debuggers")
                .unwrap();
        REGISTRY.register(Box::new(cs_cache_object_drop.clone()))?;
        let cs_cache_object_delete_fail =
            prometheus::register_counter!("cs_cache_object_delete_fail", "Cache system debuggers")
                .unwrap();
        REGISTRY.register(Box::new(cs_cache_object_delete_fail.clone()))?;
        // debug writer
        let cs_executor_finished =
            prometheus::register_counter!("cs_executor_finished", "Cache system debuggers")
                .unwrap();
        REGISTRY.register(Box::new(cs_executor_finished.clone()))?;
        let cs_executor_began =
            prometheus::register_counter!("cs_executor_began", "Cache system debuggers").unwrap();
        REGISTRY.register(Box::new(cs_executor_began.clone()))?;

        Ok(GlobalAnalytics {
            requests,
            internal_errors,
            counter: Counter {
                new: counter_new,
                inc: counter_inc,
                dec: counter_dec,
                get: counter_get,
                reset: counter_reset,
            },
            cs: CachingSystem {
                new_reader: cs_new_reader,
                no_reader_file: cs_no_reader_file,
                reader_drop: cs_reader_drop,
                reader_poll: cs_reader_poll,
                reader_poll_empty: cs_reader_poll_empty,
                reader_poll_complete: cs_reader_poll_complete,
                reader_tm_insert: cs_reader_tm_insert,
                new_writer: cs_new_writer,
                writer_create_directory: cs_writer_create_directory,
                writer_create_directory_fail: cs_writer_create_directory_fail,
                writer_write: cs_writer_write,
                writer_write_ok: cs_writer_write_ok,
                writer_write_err: cs_writer_write_err,
                writer_drop: cs_writer_drop,
                writer_drop_finish: cs_writer_drop_finish,
                writer_non_expected_length: cs_writer_non_expected_length,
                writer_attempt_insert: cs_writer_attempt_insert,
                writer_finish_insert: cs_writer_finish_insert,
                writer_finish_write: cs_writer_finish_write,
                writer_finish_write_finished: cs_writer_finish_write_finished,
                cache_object_drop: cs_cache_object_drop,
                cache_object_drop_delete_fail: cs_cache_object_delete_fail,
                executor_began: cs_executor_began,
                executor_finished: cs_executor_finished,
            },
            threading: Threading {
                ips_banned,
                ips_removed,
                inserted,
                ran,
                rows_inserted,
                rows_errored,
                rows_timedout,
            },
            tls: Tls {
                chrome,
                safari,
                firefox,
                unknown,
                cert_requested,
                cert_found,
                cert_not_found,
                cert_onpacketware,
            },
            cookie: Cookie {
                cookie_generated,
                cookie_tested,
                cookie_expired,
                cookie_failed,
                cookie_misused,
                cookie_worked,
            },
            rproxy: Rproxy {
                new_stream,
                stream_ended,
                received_traffic,
                poll_next_requests,
                larger_response_than_expected,
                cache_max_hit,
                origin_found,
                origin_not_found,
                localhost,
                rl_backend,
                is_ws,
                ip_data_requested: rproxy_ip_data_requested,
                stream_allowed,
                stream_denied,
                attempting_cache,
                sent_to_backend,
                checked_cache_headers,
                websocket_error,
                actor_start_error: actor_error,
                ping,
                text,
                binary,
                close,
                ws_method_not_allowed,
                turbo_mode_inserted,
            },
            ipdata: Ipdata {
                requested: total_ip_data_requested,
                cached: total_ip_data_cached,
                miss: total_ip_data_miss,
                ratelimited: total_ip_data_ratelimited,
                not_found: total_ip_data_notfound,
            },
            handler: Handler {
                he: HumanEngine {
                    domain_traffic: human_engine_domain_traffic,
                    headers: human_engine_headers,
                    open_proxy: human_engine_open_proxy,
                    user_agent: human_engine_user_agent,
                    tls_fingerprint: human_engine_tls_fingerprint,
                    tls_no_fingerprint: human_engine_no_tls_fingerprint,
                    open_conn_alert: human_engine_open_conn_alert,
                    request_alert: human_engine_request_alert,
                    internal_counter_alert: human_engine_internal_counter_alert,
                    passed: human_engine_passed,
                    challenged: human_engine_challenged,
                    inspected: human_engine_total,
                    multiplier_origin_errors: human_engine_multipler_origin_errors,
                    multiplier_requests: human_engine_multipler_requests,
                },
                ri: RequestInspection {
                    domain_blocked: request_inspection_domain_blocked,
                    cdn_loop: request_inspection_cdn_loop,
                    weird_user: request_inspection_weird_user,
                    header_not_string: request_inspection_header_not_string,
                    waf: request_inspection_waf,
                },
                pr: PageRules {
                    rules_tested: page_rules_rules_tested,
                    matches_tested: page_rules_matches_tested,
                    did_action: page_rules_did_action,
                    block: page_rules_action_block,
                    cache_data: page_rules_cache_data,
                    redirect: page_rules_redirect,
                    use_backend: page_rules_use_backend,
                    backend_not_found: page_rules_backend_not_found,
                    buckets: page_rules_action_bucket,
                    challenge: page_rules_action_challenge,
                    key_value_tested: page_rules_kv_tested,
                    pure_string_tested: page_rules_string_tested,
                },
                c: Caching {
                    found: cache_found,
                    expired: cache_expired,
                    metadata_existed_data_didnt: cache_metadata_existed_data_didnt,
                    cache_hit,
                    turbo_inserted: cache_turbo_inserted,
                    encoded: cache_encoded,
                },
                b: Bots {
                    ratelimited: bots_ratelimited,
                    bot: bots_detected,
                    not_bot: bots_not_detected,
                },
                ae: ApiEngine {
                    ws_no_engine,
                    settings_checked,
                    whitelist_factors_checked,
                    whitelist_ip_hit,
                    whitelist_header_hit,
                    cache: api_cache,
                    ratelimit: api_ratelimit,
                    setting_found_no_rule: setting_didnt_find_rule,
                    ratelimited: api_ratelimited,
                },
            },
            bucket: Bucket {
                public: PublicBucket {
                    new: public_bucket_new,
                    ip_inserted: public_bucket_ip_inserted,
                    bucket_inserted: public_bucket_bucket_inserted,
                    passed: public_bucket_passed,
                    ratelimited: public_bucket_ratelimited,
                    global_ratelimited: public_bucket_global_ratelimited,
                    ip_data_set: public_bucket_ip_data_set,
                    ip_data_get: public_bucket_ip_data_get,
                    expired: public_bucket_expired,
                    bucket_entry_ratelimit: public_bucket_bucket_entry_ratelimit,
                },
                global: GlobalBucket {
                    ratelimited: global_bucket_ratelimited,
                    allowed: global_bucket_allowed,
                    new: global_bucket_new,
                },
                private: PrivateBucket {
                    ratelimited: private_bucket_ratelimited,
                    allowed: private_bucket_allowed,
                    new: private_bucket_new,
                },
            },
            grpc: Grpc {
                success: grpc_success,
                fail: grpc_fail,
                update: grpc_update,
                new: grpc_new,
                delete: grpc_delete,
                challenge: grpc_challenge,
                challenge_removal: grpc_challenge_removal,
                challenge_completed: grpc_challenge_completed,
                clear_cache: grpc_clear_cache,
                challenge_shuffle: grpc_challenge_shuffle,
            },
            timeout: Timeout { stream: timeout },
            streams: Streams {
                http: streams_http,
                https: streams_https,
            },
            template: Templates {
                rl_backend: template_rl_backend,
                too_many_requests: template_too_many_requests,
                smart_challenge: template_smart_challenge,
                origin_invalid: template_origin_invalid,
                origin_down: template_origin_down,
                waf: template_waf,
                invalid: template_invalid,
                global_ratelimit: template_global_ratelimit,
                domain_not_found: template_domain_not_found,
                direct_ip: template_direct_ip,
                blocked: template_blocked,
                api_engine_blocked: template_api_engine_blocked,
                internal_error: template_internal_error,
                api_engine_required: template_api_engine_required,
            },
            cgi: Cgi {
                cgi_request,
                submit_challenge_request,
                submit_challenge_success,
                submit_challenge_fail,
                submit_challenge_timeout,
                submit_challenge_size,
                challenge_not_utf8,
                challenge_bad_base64,
                challenge_too_short,
                challenge_decryption_fail,
                challenge_decryption_not_utf8,
                challenge_unable_to_format,
                challenge_ua_no_match,
                challenge_timeout_replay,
                challenge_browser_no_match,
                challenge_notifications_not_persistent,
                challenge_fixed_memory_set,
                challenge_audio_context,
                challenge_chrome,
                challenge_firefox,
                challenge_safari,
                challenge_unknown,
                well_known,
                well_known_token_found,
                well_known_token_not_found,
            },
        })
    }
}

pub struct GlobalAnalytics {
    pub requests: PCounter,
    pub internal_errors: PCounter,
    // open tasks
    // pub tasks: Tasks,
    pub counter: Counter,
    pub threading: Threading,
    pub tls: Tls,
    pub cookie: Cookie,
    pub rproxy: Rproxy,
    pub ipdata: Ipdata,
    pub handler: Handler,
    pub bucket: Bucket,
    pub grpc: Grpc,
    pub timeout: Timeout,
    pub streams: Streams,
    pub template: Templates,
    pub cgi: Cgi,
    pub cs: CachingSystem,
}

pub struct CachingSystem {
    // reader
    pub new_reader: PCounter,
    pub no_reader_file: PCounter,
    pub reader_drop: PCounter,
    pub reader_poll: PCounter,
    pub reader_poll_empty: PCounter,
    pub reader_poll_complete: PCounter,
    pub reader_tm_insert: PCounter,

    // writer
    pub new_writer: PCounter,
    pub writer_create_directory: PCounter,
    pub writer_create_directory_fail: PCounter,
    pub writer_write: PCounter,
    pub writer_write_ok: PCounter,
    pub writer_write_err: PCounter,
    pub writer_drop: PCounter,
    pub writer_drop_finish: PCounter,
    pub writer_non_expected_length: PCounter,
    // specifically suspecting dashmap as being the culprit
    pub writer_attempt_insert: PCounter,
    pub writer_finish_insert: PCounter,
    // for the finish_write dashmap debugging
    pub writer_finish_write: PCounter,
    pub writer_finish_write_finished: PCounter,

    // cached object
    pub cache_object_drop: PCounter,
    pub cache_object_drop_delete_fail: PCounter,

    // debug writer
    pub executor_began: PCounter,
    pub executor_finished: PCounter,
}

pub struct Tasks {
    pub count: PCounter,
    pub active: PCounter,
    pub parked: PCounter,
    pub stolen: PCounter,
    pub overflow: PCounter,
    pub depth: PCounter,
}

pub struct Cgi {
    pub cgi_request: PCounter,
    pub submit_challenge_request: PCounter,
    pub submit_challenge_success: PCounter,
    pub submit_challenge_fail: PCounter,

    pub submit_challenge_timeout: PCounter,
    pub submit_challenge_size: PCounter,

    // errors
    pub challenge_not_utf8: PCounter,
    pub challenge_bad_base64: PCounter,
    pub challenge_too_short: PCounter,
    pub challenge_decryption_fail: PCounter,
    pub challenge_decryption_not_utf8: PCounter,
    // the worst one
    pub challenge_unable_to_format: PCounter,

    // challenge being analyzed errors
    pub challenge_ua_no_match: PCounter,
    pub challenge_timeout_replay: PCounter,
    pub challenge_browser_no_match: PCounter,

    // statistics
    pub challenge_notifications_not_persistent: PCounter,
    pub challenge_fixed_memory_set: PCounter,
    pub challenge_audio_context: PCounter,

    // browser cookie issued to
    pub challenge_chrome: PCounter,
    pub challenge_firefox: PCounter,
    pub challenge_safari: PCounter,
    pub challenge_unknown: PCounter,

    pub well_known: PCounter,
    pub well_known_token_found: PCounter,
    pub well_known_token_not_found: PCounter,
}

pub struct Streams {
    pub http: PCounter,
    pub https: PCounter,
}

pub struct Timeout {
    pub stream: PCounter,
}

pub struct Grpc {
    pub success: PCounter,
    pub fail: PCounter,
    pub update: PCounter,
    pub new: PCounter,
    pub delete: PCounter,
    pub challenge: PCounter,
    pub challenge_removal: PCounter,
    pub challenge_completed: PCounter,
    pub clear_cache: PCounter,
    pub challenge_shuffle: PCounter,
}

pub struct Counter {
    pub new: PCounter,
    pub inc: PCounter,
    pub dec: PCounter,
    pub get: PCounter,
    pub reset: PCounter,
}

pub struct Threading {
    pub ips_banned: PCounter,
    pub ips_removed: PCounter,
    pub inserted: PCounter,
    pub ran: PCounter,

    // analytics
    pub rows_inserted: PCounter,
    pub rows_errored: PCounter,
    pub rows_timedout: PCounter,
}

pub struct Tls {
    pub chrome: PCounter,
    pub safari: PCounter,
    pub firefox: PCounter,
    pub unknown: PCounter,

    pub cert_requested: PCounter,
    pub cert_found: PCounter,
    pub cert_not_found: PCounter,
    pub cert_onpacketware: PCounter,
}

pub struct Cookie {
    pub cookie_generated: PCounter,
    pub cookie_tested: PCounter,
    pub cookie_expired: PCounter,
    pub cookie_failed: PCounter,
    pub cookie_misused: PCounter,
    pub cookie_worked: PCounter,
}

pub struct Rproxy {
    pub new_stream: PCounter,
    pub stream_ended: PCounter,
    pub received_traffic: PCounter,
    pub poll_next_requests: PCounter,
    pub larger_response_than_expected: PCounter,
    pub cache_max_hit: PCounter,

    pub origin_found: PCounter,
    pub origin_not_found: PCounter,
    pub localhost: PCounter,
    pub rl_backend: PCounter,
    pub is_ws: PCounter,

    pub ip_data_requested: PCounter,
    pub stream_allowed: PCounter,
    pub stream_denied: PCounter,
    pub attempting_cache: PCounter,
    pub sent_to_backend: PCounter,
    pub checked_cache_headers: PCounter,

    pub websocket_error: PCounter,
    pub actor_start_error: PCounter,

    pub ping: PCounter,
    pub text: PCounter,
    pub binary: PCounter,
    pub close: PCounter,
    pub ws_method_not_allowed: PCounter,

    pub turbo_mode_inserted: PCounter,
}

pub struct Ipdata {
    pub requested: PCounter,
    pub cached: PCounter,
    pub miss: PCounter,
    pub ratelimited: PCounter,
    pub not_found: PCounter,
}

pub struct Handler {
    pub he: HumanEngine,
    pub ri: RequestInspection,
    pub pr: PageRules,
    pub c: Caching,
    pub b: Bots,
    pub ae: ApiEngine,
}

pub struct HumanEngine {
    pub domain_traffic: PCounter,
    pub headers: PCounter,
    pub open_proxy: PCounter,
    pub user_agent: PCounter,
    pub tls_fingerprint: PCounter,
    pub tls_no_fingerprint: PCounter,
    pub open_conn_alert: PCounter,
    pub request_alert: PCounter,
    pub internal_counter_alert: PCounter,
    pub multiplier_requests: PCounter,
    pub multiplier_origin_errors: PCounter,

    // aggregates
    pub inspected: PCounter,
    pub passed: PCounter,
    pub challenged: PCounter,
}

pub struct RequestInspection {
    pub domain_blocked: PCounter,
    pub cdn_loop: PCounter,
    pub weird_user: PCounter,
    pub header_not_string: PCounter,
    pub waf: PCounter,
}

pub struct PageRules {
    pub rules_tested: PCounter,
    pub matches_tested: PCounter,
    pub did_action: PCounter,
    pub block: PCounter,
    pub cache_data: PCounter,
    pub redirect: PCounter,
    pub use_backend: PCounter,
    pub backend_not_found: PCounter,
    pub buckets: PCounter,
    pub challenge: PCounter,
    pub key_value_tested: PCounter,
    pub pure_string_tested: PCounter,
}

pub struct Caching {
    pub found: PCounter,
    pub expired: PCounter,
    pub metadata_existed_data_didnt: PCounter,
    pub cache_hit: PCounter,
    pub turbo_inserted: PCounter,
    pub encoded: PCounter,
}

pub struct Bots {
    pub ratelimited: PCounter,
    pub bot: PCounter,
    pub not_bot: PCounter,
}

pub struct ApiEngine {
    pub ws_no_engine: PCounter,
    pub settings_checked: PCounter,
    pub whitelist_factors_checked: PCounter,
    pub whitelist_ip_hit: PCounter,
    pub whitelist_header_hit: PCounter,
    pub cache: PCounter,
    pub ratelimit: PCounter,
    pub setting_found_no_rule: PCounter,
    pub ratelimited: PCounter,
}

pub struct Bucket {
    pub public: PublicBucket,
    pub global: GlobalBucket,
    pub private: PrivateBucket,
}

pub struct GlobalBucket {
    pub ratelimited: PCounter,
    pub allowed: PCounter,
    pub new: PCounter,
}

pub struct PrivateBucket {
    pub ratelimited: PCounter,
    pub allowed: PCounter,
    pub new: PCounter,
}

pub struct PublicBucket {
    pub new: PCounter,
    pub ip_inserted: PCounter,
    pub bucket_inserted: PCounter,
    pub passed: PCounter,
    pub ratelimited: PCounter,
    pub global_ratelimited: PCounter,
    pub ip_data_set: PCounter,
    pub ip_data_get: PCounter,
    pub expired: PCounter,
    pub bucket_entry_ratelimit: PCounter,
}

pub struct Templates {
    pub rl_backend: PCounter,
    pub too_many_requests: PCounter,
    pub smart_challenge: PCounter,
    pub origin_invalid: PCounter,
    pub origin_down: PCounter,
    pub waf: PCounter,
    pub invalid: PCounter,
    pub global_ratelimit: PCounter,
    pub domain_not_found: PCounter,
    pub direct_ip: PCounter,
    pub blocked: PCounter,
    pub api_engine_blocked: PCounter,
    pub internal_error: PCounter,
    pub api_engine_required: PCounter,
}
