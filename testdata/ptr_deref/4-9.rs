pub unsafe extern "C" fn wasmtime_module_new(
    engine: &wasm_engine_t,
    wasm: *const u8,
    len: usize,
    out: &mut *mut wasmtime_module_t,
) -> Option<Box<wasmtime_error_t>> {
    handle_result(
        Module::from_binary(&engine.engine, crate::slice_from_raw_parts(wasm, len)),
        |module| {
            *out = Box::into_raw(Box::new(wasmtime_module_t { module }));
        },
    )
}

//https://github.com/pingiun/wasmtime/blob/3f72edaf935c35e6b3147d679806579f4dda65e7/crates/c-api/src/module.rs#L205