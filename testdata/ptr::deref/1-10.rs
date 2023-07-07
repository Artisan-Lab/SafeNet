fn napi_create_array(env: *mut Env, result: *mut napi_value) -> napi_status {
    check_env!(env);
    check_arg!(env, result);
    let env = unsafe { &mut *env };
    *result = v8::Array::new(&mut env.scope(), 0).into();
    napi_ok
}
ror_code
}
// https://github.com/denoland/deno/blob/57fae55d822f5aae52ea93d0e55155bc9c12672f/cli/napi/js_native_api.rs#L2010