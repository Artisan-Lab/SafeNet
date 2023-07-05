pub extern "C" fn wasm_module_new(
    store: &wasm_store_t,
    binary: &wasm_byte_vec_t,
) -> Option<Box<wasm_module_t>> {
    let mut ret = ptr::null_mut();
    match wasmtime_module_new(store, binary, &mut ret) {
        Some(_err) => None,
        None => {
            assert!(!ret.is_null());
            Some(unsafe { Box::from_raw(ret) })
        }
    }
}


// https://github.com/osa1/wasmtime/blob/51f9ac2150bd880dc7b92952ba92e72ea3ab60b7/crates/c-api/src/module.rs#L33