pub unsafe extern "C" fn wasmtime_func_call(
    mut store: CStoreContextMut<'_>,
    func: &Func,
    args: *const wasmtime_val_t,
    nargs: usize,
    results: *mut MaybeUninit<wasmtime_val_t>,
    nresults: usize,
    trap_ret: &mut *mut wasm_trap_t,
) -> Option<Box<wasmtime_error_t>> {
    let mut store = store.as_context_mut();
    let mut params = mem::take(&mut store.data_mut().wasm_val_storage);
    let (wt_params, wt_results) = translate_args(
        &mut params,
        crate::slice_from_raw_parts(args, nargs)
            .iter()
            .map(|i| i.to_val()),
        nresults,
    );

    // We're calling arbitrary code here most of the time, and we in general
    // want to try to insulate callers against bugs in wasmtime/wasi/etc if we
    // can. As a result we catch panics here and transform them to traps to
    // allow the caller to have any insulation possible against Rust panics.
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        func.call(&mut store, wt_params, wt_results)
    }));
    match result {
        Ok(Ok(())) => {
            let results = crate::slice_from_raw_parts_mut(results, nresults);
            for (slot, val) in results.iter_mut().zip(wt_results.iter()) {
                crate::initialize(slot, wasmtime_val_t::from_val(val.clone()));
            }
            params.truncate(0);
            store.data_mut().wasm_val_storage = params;
            None
        }
        Ok(Err(trap)) => store_err(trap, trap_ret),
        Err(panic) => {
            let err = error_from_panic(panic);
            *trap_ret = Box::into_raw(Box::new(wasm_trap_t::new(err)));
            None
        }
    }
}
//https://github.com/bytecodealliance/wasmtime/blob/6a868ef181b481076b01b0e59c05aa8f5fae1abb/crates/c-api/src/func.rs#L357