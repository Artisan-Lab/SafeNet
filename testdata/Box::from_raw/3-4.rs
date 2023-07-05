pub(crate) fn from_jsvalue(ctx: &JSContext, val: JSValue) -> Self {
    let obj_val = val.to_object(ctx).unwrap();
    let wasmer_error_ptr = obj_val.get_property(&ctx, "wasmer_error_ptr".to_string());
    if wasmer_error_ptr.is_number(ctx) {
        let err_ptr = wasmer_error_ptr.to_number(ctx).unwrap() as usize
            as *mut Box<dyn Error + Send + Sync>;
        let err = unsafe { Box::from_raw(err_ptr) };
        return Self::user(*err);
    }
    Self {
        inner: InnerTrap::JSC(val),
    }
}

// https://github.com/wasmerio/wasmer/blob/f9ce24d504c7a14f27663741c235d80da60b1830/lib/api/src/jsc/trap.rs#L75