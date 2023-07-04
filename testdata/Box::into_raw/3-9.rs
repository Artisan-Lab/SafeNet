pub unsafe extern "C" fn wasm_table_new(
    store: &wasm_store_t,
    tt: &wasm_tabletype_t,
    init: *mut wasm_ref_t,
) -> Option<Box<wasm_table_t>> {
    let init: Val = if !init.is_null() {
        Box::from_raw(init).r.into()
    } else {
        Val::AnyRef(AnyRef::Null)
    };
    let table = Table::new(&store.store.borrow(), tt.ty().ty.clone(), init).ok()?;
    Some(Box::new(wasm_table_t {
        ext: wasm_extern_t {
            which: ExternHost::Table(HostRef::new(table)),
        },
    }))
}