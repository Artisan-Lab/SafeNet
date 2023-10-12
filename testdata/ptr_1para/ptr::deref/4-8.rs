pub unsafe extern "C" fn wasmtime_module_deserialize_file(
    engine: &wasm_engine_t,
    path: *const c_char,
    out: &mut *mut wasmtime_module_t,
) -> Option<Box<wasmtime_error_t>> {
    let path = CStr::from_ptr(path);
    let result = path
        .to_str()
        .context("input path is not valid utf-8")
        .and_then(|path| Module::deserialize_file(&engine.engine, path));
    handle_result(result, |module| {
        *out = Box::into_raw(Box::new(wasmtime_module_t { module }));
    })
}
//https://github.com/pingiun/wasmtime/blob/3f72edaf935c35e6b3147d679806579f4dda65e7/crates/c-api/src/module.rs#L205