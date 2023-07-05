fn napi_create_buffer_copy(
    env: *mut Env,
    len: usize,
    data: *mut u8,
    result_data: *mut *mut u8,
    result: *mut napi_value,
  ) -> napi_status {
    check_env!(env);
    let env = unsafe { &mut *env };
    let value = v8::ArrayBuffer::new(&mut env.scope(), len);
    let ptr = get_array_buffer_ptr(value);
    std::ptr::copy(data, ptr, len);
    if !result_data.is_null() {
      *result_data = ptr;
    }
    let value = v8::Uint8Array::new(&mut env.scope(), value, 0, len).unwrap();
    let value: v8::Local<v8::Value> = value.into();
    *result = value.into();
    napi_ok
  }
  // https://github.com/denoland/deno/blob/cd2525d4cff4e18b1e3e7d29458079942ba2b6c5/cli/napi/js_native_api.rs#L277