pub(crate) unsafe extern "C" fn free_array(p: *mut c_void) {
    drop(Rc::from_raw(p as *const Vec<Value>));
}