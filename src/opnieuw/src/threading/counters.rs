pub fn analytic_loop() {
    // total requests made to our service
    let total_reqs: GenericCounter<AtomicF64> =
        PromCount::with_opts(Opts::new("total_reqs", "Total requests")).unwrap();

    // requests made directly to our IPs
    let ip_reqs: GenericCounter<AtomicF64> =
        PromCount::with_opts(Opts::new("ip_reqs", "Requests made directly to our IP")).unwrap();

    // general errors
    let err_reqs: GenericCounter<AtomicF64> =
        PromCount::with_opts(Opts::new("err_reqs", "Requests that resulted in an error")).unwrap();

    // global ratelimit
    let global_rl: GenericCounter<AtomicF64> = PromCount::with_opts(Opts::new(
        "global_rl_reqs",
        "Requests that came from sources which hit our limit",
    ))
    .unwrap();

    // requests that are hitting the cgi paths
    let cgi_reqs: GenericCounter<AtomicF64> =
        PromCount::with_opts(Opts::new("cgi_reqs", "Requests that hit the CGI resources")).unwrap();

    // requests that are hitting the cgi paths
    let domain_not_found_reqs: GenericCounter<AtomicF64> = PromCount::with_opts(Opts::new(
        "domain_not_found_reqs",
        "Requests that requested a host which is not in our database",
    ))
    .unwrap();
}
