unsafe extern "C" fn free_func(_contents: *mut c_void, free_user_data: *mut c_void) {
    let _ = Rc::from_raw(free_user_data as _);
}
