pub extern "C" fn backing_store_deleter_callback(
    data: *mut c_void,
    byte_length: usize,
    _deleter_data: *mut c_void,
  ) {
    let slice_ptr = ptr::slice_from_raw_parts_mut(data as *mut u8, byte_length);
    let b = unsafe { Box::from_raw(slice_ptr) };
    drop(b);
  }
  
// https://github.com/denogdev/denog/blob/29dade113e7d7e7d1ececace506e8b1a47eeca95/cli/napi/js_native_api.rs#L535