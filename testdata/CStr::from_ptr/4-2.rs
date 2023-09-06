pub extern "C" fn metrics_run(
    bind_address: *const c_char,
    allow_ips: *const *const c_char,
    allow_ips_len: usize,
    prometheus_port: u16,
    debug_metrics: bool,
) -> bool {
    // Convert the C string IPs to Rust strings.
    let allow_ips = unsafe { slice::from_raw_parts(allow_ips, allow_ips_len) };
    let mut allow_ips: Vec<&str> = match allow_ips
        .iter()
        .map(|&p| unsafe { CStr::from_ptr(p) })
        .map(|s| s.to_str().ok())
        .collect()
    {
        Some(ips) => ips,
        None => {
            return false;
        }
    };
    // We always allow localhost.
    allow_ips.extend(["127.0.0.0/8", "::1/128"]);

    // Parse the address to bind to.
    let bind_address = SocketAddr::new(
        if allow_ips.is_empty() {
            // Default to loopback if not allowing external IPs.
            "127.0.0.1".parse::<IpAddr>().unwrap()
        } else if bind_address.is_null() {
            // No specific bind address specified, bind to any.
            "0.0.0.0".parse::<IpAddr>().unwrap()
        } else {
            match unsafe { CStr::from_ptr(bind_address) }
                .to_str()
                .ok()
                .and_then(|s| s.parse::<IpAddr>().ok())
            {
                Some(addr) => addr,
                None => {
                    error!("Invalid -metricsbind argument");
                    return false;
                }
            }
        },
        prometheus_port,
    );

    // Metrics matching any of these filters will be discarded.
    let filters = if debug_metrics {
        &[][..]
    } else {
        &["zcashd.debug."]
    };

    allow_ips
        .into_iter()
        .try_fold(
            PrometheusBuilder::new().with_http_listener(bind_address),
            |builder, subnet| {
                builder.add_allowed_address(subnet).map_err(|e| {
                    error!("Invalid -metricsallowip argument '{}': {}", subnet, e);
                    e
                })
            },
        )
        .and_then(|builder| {
            metrics_install(builder, filters).map_err(|e| {
                error!("Could not install Prometheus exporter: {}", e);
                e
            })
        })
        .is_ok()
}