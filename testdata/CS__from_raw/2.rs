pub fn free_cstr(ptr: *const c_char) {
    let _ = unsafe { std::ffi::CString::from_raw(ptr as *mut _) };
}
/*
https://github.com/radareorg/r2pipe.rs/blob/1208b28183cf3b24310955d19715729fe4a529d1/src/dlfcn.rs#L14
*/