  /*
  https://github.com/denoland/deno/blob/4a18c761351dccb146973793cf22e6efffff18bf/cli/napi/js_native_api.rs#L781
  */

fn napi_create_string_latin1(
    env: *mut Env,
    string: *const u8,
    length: usize,
    result: *mut napi_value,
  ) -> napi_status {
    check_env!(env);
    let env = unsafe { &mut *env };
    if length > 0 {
      check_arg!(env, string);
    }
    check_arg!(env, result);
    return_status_if_false!(
      env,
      (length == NAPI_AUTO_LENGTH) || length <= INT_MAX as _,
      napi_invalid_arg
    );
  
    let string = if length == NAPI_AUTO_LENGTH {
      std::ffi::CStr::from_ptr(string as *const _)
        .to_str()
        .unwrap()
        .as_bytes()
    } else {
      std::slice::from_raw_parts(string, length)
    };
    let Some(v8str) = v8::String::new_from_one_byte(
      &mut env.scope(),
      string,
      v8::NewStringType::Normal,
    ) else {
      return napi_generic_failure;
    };
    *result = v8str.into();
  
    napi_ok
  }