pub extern "C" fn wasmtime_funcref_table_get(
    t: &wasm_table_t,
    index: wasm_table_size_t,
    ptr: &mut *mut wasm_func_t,
) -> bool {
    match t.table().get(index) {
        Some(val) => {
            *ptr = match val {
                Val::FuncRef(None) => ptr::null_mut(),
                Val::FuncRef(Some(f)) => Box::into_raw(Box::new(f.into())),
                _ => return false,
            };
        }

        _ => return false,
    }
    true
}
// https://github.com/willemneal/wasmtime/blob/a9455a8e5188ba70a2831279b5a3968e2c192539/crates/c-api/src/table.rs#L106