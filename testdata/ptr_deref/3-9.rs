extern "C" fn call_fn(info: *const v8::FunctionCallbackInfo) {
    let info = unsafe { &*info };
    let args = v8::FunctionCallbackArguments::from_function_callback_info(info);
    let mut rv = v8::ReturnValue::from_function_callback_info(info);
    // SAFETY: create_function guarantees that the data is a CallbackInfo external.
    let info_ptr: *mut CallbackInfo = unsafe {
        let external_value = v8::Local::<v8::External>::cast(args.data());
        external_value.value() as _
    };

    // SAFETY: pointer from Box::into_raw.
    let mut info = unsafe { &mut *info_ptr };
    info.args = &args as *const _ as *const c_void;

    if let Some(f) = info.cb {
        // SAFETY: calling user provided function pointer.
        let value = unsafe { f(info.env, info_ptr as *mut _) };
        // SAFETY: napi_value is represented as v8::Local<v8::Value> internally.
        rv.set(unsafe { transmute::<napi_value, v8::Local<v8::Value>>(value) });
    }
}

//https://github.com/denoland/deno/blob/57fae55d822f5aae52ea93d0e55155bc9c12672f/ext/napi/function.rs#L39