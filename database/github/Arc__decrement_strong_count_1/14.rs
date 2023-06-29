pub extern "C" fn #free_fn_ident(
    ptr: *const ::std::ffi::c_void,
    call_status: &mut ::uniffi::RustCallStatus
) {
    uniffi::rust_call(call_status, || {
        assert!(!ptr.is_null());
        let ptr = ptr.cast::<#ident>();
        unsafe {
            ::std::sync::Arc::decrement_strong_count(ptr);
        }
        Ok(())
    });
}
/*
https://github.com/mozilla/uniffi-rs/blob/6ac168b91df5853f5cb77b36b35b59a42cc4c80f/uniffi_macros/src/object.rs#L35
*/