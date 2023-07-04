fn raw(b: &mut Box<Self>) -> *mut libc::c_void {
    let ptr: *mut Self = b.as_mut();
    ptr as *mut libc::c_void
}