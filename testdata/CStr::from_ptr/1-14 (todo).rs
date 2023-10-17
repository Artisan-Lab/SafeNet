pub extern "C" fn tracing_init(
    #[cfg(not(target_os = "windows"))] log_path: *const u8,
    #[cfg(target_os = "windows")] log_path: *const u16,
    log_path_len: usize,
    initial_filter: *const c_char,
    log_timestamps: bool,
) -> *mut TracingHandle {
    let initial_filter = unsafe { CStr::from_ptr(initial_filter) }
        .to_str()
        .expect("initial filter should be a valid string");

    let log_path = if log_path.is_null() {
        None
    } else {
        Some(unsafe { slice::from_raw_parts(log_path, log_path_len) })
    };

    #[cfg(not(target_os = "windows"))]
    let log_path = log_path.map(OsStr::from_bytes);

    #[cfg(target_os = "windows")]
    let log_path = log_path.map(OsString::from_wide);

    let log_path = log_path.as_ref().map(Path::new);

    let (file_logger, file_no_timestamps, file_guard) = if let Some(log_path) = log_path {
        let file_appender = tracing_appender::rolling::never(
            log_path.parent().unwrap(),
            log_path.file_name().unwrap(),
        );
        let (non_blocking, file_guard) = tracing_appender::non_blocking(file_appender);

        if log_timestamps {
            (
                Some(
                    tracing_subscriber::fmt::layer()
                        .with_ansi(false)
                        .with_writer(non_blocking),
                ),
                None,
                Some(file_guard),
            )
        } else {
            (
                None,
                Some(
                    tracing_subscriber::fmt::layer()
                        .with_ansi(false)
                        .with_writer(non_blocking)
                        .without_time(),
                ),
                Some(file_guard),
            )
        }
    } else {
        (None, None, None)
    };

    let (stdout_logger, stdout_no_timestamps) = if file_logger.is_none() {
        if log_timestamps {
            (Some(tracing_subscriber::fmt::layer().with_ansi(true)), None)
        } else {
            (
                None,
                Some(
                    tracing_subscriber::fmt::layer()
                        .with_ansi(true)
                        .without_time(),
                ),
            )
        }
    } else {
        (None, None)
    };

    let (filter, reload_handle) = reload::Layer::new(EnvFilter::from(initial_filter));

    tracing_subscriber::registry()
        .with(stdout_logger)
        .with(stdout_no_timestamps)
        .with(file_logger)
        .with(file_no_timestamps)
        .with(filter)
        .init();

    Box::into_raw(Box::new(TracingHandle {
        _file_guard: file_guard,
        reload_handle: Box::new(reload_handle),
    }))
}