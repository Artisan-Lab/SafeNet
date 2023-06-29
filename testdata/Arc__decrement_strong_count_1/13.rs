pub unsafe extern "C" fn decrement_arc(arc_ptr: u64) {
    Arc::decrement_strong_count(arc_ptr as usize as *const Vec<u8>);
}
/*
https://github.com/swyxio/zaplib/blob/55820affcee3f2761e0e2f9e799fdf039874610a/zaplib/main/src/cx_wasm32.rs#L854
*/