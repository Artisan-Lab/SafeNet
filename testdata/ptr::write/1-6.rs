pub(crate) unsafe fn write_value_to(self, store: &Store, p: *mut u128) {
    match self {
        Val::I32(i) => ptr::write(p as *mut i32, i),
        Val::I64(i) => ptr::write(p as *mut i64, i),
        Val::F32(u) => ptr::write(p as *mut u32, u),
        Val::F64(u) => ptr::write(p as *mut u64, u),
        Val::V128(b) => ptr::write(p as *mut u128, b),
        Val::ExternRef(None) => ptr::write(p, 0),
        Val::ExternRef(Some(x)) => {
            let externref_ptr = x.inner.as_raw();
            store
                .externref_activations_table()
                .insert_with_gc(x.inner, store.stack_map_registry());
            ptr::write(p as *mut *mut u8, externref_ptr)
        }
        Val::FuncRef(f) => ptr::write(
            p as *mut *mut runtime::VMCallerCheckedAnyfunc,
            if let Some(f) = f {
                f.caller_checked_anyfunc().as_ptr()
            } else {
                ptr::null_mut()
            },
        ),
    }
}
// https://github.com/Kong/wasmtime/blob/f2f770626511c08c632c52e5f868ac13170687be/crates/wasmtime/src/values.rs#L104