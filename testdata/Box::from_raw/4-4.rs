pub extern "C" fn tracing_free(handle: *mut TracingHandle) {
    drop(unsafe { Box::from_raw(handle) });
}

// https://github.com/LayerTwo-Labs/zcash-sidechain/blob/93db677b91ced8accdf67ed796f06275d09c8af2/src/rust/src/tracing_ffi.rs#L193