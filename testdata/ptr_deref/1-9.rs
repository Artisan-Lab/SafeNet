pub(crate) fn napi_set_last_error(
    env: *mut Env,
    error_code: napi_status,
    engine_error_code: i32,
    engine_reserved: *mut c_void,
) -> napi_status {
    let env = unsafe { &mut *env };
    env.last_error.error_code = error_code;
    env.last_error.engine_error_code = engine_error_code;
    env.last_error.engine_reserved = engine_reserved;
    error_code
}
// https://github.com/denoland/deno/blob/57fae55d822f5aae52ea93d0e55155bc9c12672f/cli/napi/js_native_api.rs#L2010