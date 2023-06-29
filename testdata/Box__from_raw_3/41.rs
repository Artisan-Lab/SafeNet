pub extern "C" fn backing_store_deleter_callback(
    data: *mut c_void,
    _byte_length: usize,
    deleter_data: *mut c_void,
  ) {
    let mut finalizer =
      unsafe { Box::from_raw(deleter_data as *mut BufferFinalizer) };
  
    finalizer.finalize_data = data;
  }
/*
https://github.com/denoland/deno/blob/f81027ae9f5646ddfa06046836001aaa726c4025/cli/napi/js_native_api.rs#L577
*/