pub extern "C" fn unified_full_viewing_key_free(key: *mut Ufvk) {
    if !key.is_null() {
        drop(unsafe { Box::from_raw(key) });
    }
}

// https://github.com/LayerTwo-Labs/zcash-sidechain/blob/93db677b91ced8accdf67ed796f06275d09c8af2/src/rust/src/unified_keys_ffi.rs#L13