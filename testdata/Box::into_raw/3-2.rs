 pub fn enable(enabled: bool) {
        let mut lock = CONTEXT_SEND.addr.lock().unwrap();
        if enabled {
            if *lock == 0 {
                match crate::create_cliprdr_context(
                    true,
                    false,
                    CLIPBOARD_RESPONSE_WAIT_TIMEOUT_SECS,
                ) {
                    Ok(context) => {
                        log::info!("clipboard context for file transfer created.");
                        *lock = Box::into_raw(context) as _;
                    }
                    Err(err) => {
                        log::error!(
                            "Create clipboard context for file transfer: {}",
                            err.to_string()
                        );
                    }
                }
            }
        } else {
            if *lock != 0 {
                unsafe {
                    let _ = Box::from_raw(*lock as *mut CliprdrClientContext);
                }
                log::info!("clipboard context for file transfer destroyed.");
                *lock = 0;
            }
        }
    }