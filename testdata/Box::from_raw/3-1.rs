pub extern "C" fn backing_store_deleter_callback(
  data: *mut c_void,
  _byte_length: usize,
  deleter_data: *mut c_void,
) {
  let mut finalizer =
    unsafe { Box::from_raw(deleter_data as *mut BufferFinalizer) };

  finalizer.finalize_data = data;
}

// https://github.com/denoland/deno/blob/aaabff710f756f7dc8651b8e92e4cc5147830e49/cli/napi/js_native_api.rs#L577