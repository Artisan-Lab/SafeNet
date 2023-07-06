pub unsafe extern "C" fn wasmtime_module_deserialize(
    engine: &wasm_engine_t,
    bytes: *const u8,
    len: usize,
    out: &mut *mut wasmtime_module_t,
) -> Option<Box<wasmtime_error_t>> {
    let bytes = crate::slice_from_raw_parts(bytes, len);
    handle_result(Module::deserialize(&engine.engine, bytes), |module| {
        *out = Box::into_raw(Box::new(wasmtime_module_t { module }));
    })
}
//https://github.com/pingiun/wasmtime/blob/3f72edaf935c35e6b3147d679806579f4dda65e7/crates/c-api/src/module.rs#L205