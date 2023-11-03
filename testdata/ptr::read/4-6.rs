pub fn op_ffi_read_u8<FP>(
    state: &mut deno_core::OpState,
    ptr: usize,
    offset: usize,
  ) -> Result<u32, AnyError>
  where
    FP: FfiPermissions + 'static,
  {
    check_unstable(state, "Deno.UnsafePointerView#getUint8");
  
    let permissions = state.borrow_mut::<FP>();
    permissions.check(None)?;
  
    let ptr = ptr as *const c_void;
  
    if ptr.is_null() {
      return Err(type_error("Invalid u8 pointer, pointer is null"));
    }
  
    // SAFETY: ptr and offset are user provided.
    Ok(unsafe { ptr::read_unaligned::<u8>(ptr.add(offset) as *const u8) as u32 })
}
  // https://github.com/RaptorFX-JS/deno_minus_v8/blob/7d2fa400e05de1edfb4786ac0e23ea183a56c931/ext/ffi/repr.rs#L203