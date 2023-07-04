fn napi_release_threadsafe_function(
    tsfn: napi_threadsafe_function,
    _mode: napi_threadsafe_function_release_mode,
  ) -> napi_status {
    let tsfn: Box<TsFn> = Box::from_raw(tsfn as *mut TsFn);
    tsfn.release()
  }

// https://github.com/denoland/deno/blob/aaabff710f756f7dc8651b8e92e4cc5147830e49/cli/napi/threadsafe_functions.rs#L245