pub unsafe extern "C" fn wasm_instance_new(
    store: &wasm_store_t,
    wasm_module: &wasm_module_t,
    imports: *const Box<wasm_extern_t>,
    result: Option<&mut *mut wasm_trap_t>,
) -> Option<Box<wasm_instance_t>> {
    let mut instance = ptr::null_mut();
    let mut trap = ptr::null_mut();
    let err = wasmtime_instance_new(
        store,
        wasm_module,
        imports,
        wasm_module.imports.len(),
        &mut instance,
        &mut trap,
    );
    match err {
        Some(err) => {
            assert!(trap.is_null());
            assert!(instance.is_null());
            if let Some(result) = result {
                *result = Box::into_raw(err.to_trap());
            }
            None
        }
        None => {
            if instance.is_null() {
                assert!(!trap.is_null());
                if let Some(result) = result {
                    *result = trap;
                } else {
                    drop(Box::from_raw(trap))
                }
                None
            } else {
                assert!(trap.is_null());
                Some(Box::from_raw(instance))
            }
        }
    }
}