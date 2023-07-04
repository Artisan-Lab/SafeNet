pub extern "C" fn metrics_increment_counter(key: *mut FfiKey, value: u64) {
    if let Some(recorder) = try_recorder() {
        if !key.is_null() {
            let key = unsafe { Box::from_raw(key) };
            recorder.register_counter(&key.inner).increment(value);
        }
    }
}

// https://github.com/QED-it/zcash/blob/aa2f6aed7624c1e144b2f583d5b4c5783b7fe482/src/rust/src/metrics_ffi.rs#L203