extern "C" fn retain_context(ctx_ptr: *const c_void) -> *const c_void where Self: Sized  {
    // println!("ManagerContext::retain_context {:?}", ctx_ptr);
    unsafe { Arc::increment_strong_count(ctx_ptr as *mut Self); }
    ctx_ptr
}
/*
https://github.com/dashpay/dash-shared-core/blob/4466d0a4e58a1eba4f0bf3984eb491fd5595caa1/reachability/src/lib.rs#L148
*/