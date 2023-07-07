fn check_new_from_utf8_len<'s>(
    env: *mut Env,
    str_: *const c_char,
    len: usize,
) -> Result<v8::Local<'s, v8::String>, napi_status> {
    return_error_status_if_false!(
    env,
    (len == NAPI_AUTO_LENGTH) || len <= INT_MAX as _,
    napi_invalid_arg
  );
    return_error_status_if_false!(env, !str_.is_null(), napi_invalid_arg);
    let string = if len == NAPI_AUTO_LENGTH {
        let result = unsafe { std::ffi::CStr::from_ptr(str_ as *const _) }.to_str();
        return_error_status_if_false!(env, result.is_ok(), napi_generic_failure);
        result.unwrap()
    } else {
        let string = unsafe { std::slice::from_raw_parts(str_ as *const u8, len) };
        let result = std::str::from_utf8(string);
        return_error_status_if_false!(env, result.is_ok(), napi_generic_failure);
        result.unwrap()
    };
    let result = {
        let env = unsafe { &mut *env };
        v8::String::new(&mut env.scope(), string)
    };
    return_error_status_if_false!(env, result.is_some(), napi_generic_failure);
    Ok(result.unwrap())
}
// https://github.com/denoland/deno/blob/57fae55d822f5aae52ea93d0e55155bc9c12672f/cli/napi/js_native_api.rs#L2010