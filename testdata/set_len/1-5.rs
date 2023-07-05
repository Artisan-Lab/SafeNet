/*
https://github.com/softwarearchitect817/Deno-modern-runtime-JavaScript/blob/55fb564a19bd11e3841bf0d010fc2b0269c8b7c1/ext/ffi/tcc.rs#L101
*/

 pub fn relocate_and_get_symbol(
    &mut self,
    sym: &CStr,
  ) -> Result<*mut c_void, ()> {
    // SAFETY: pass null ptr to get required length
    let len = unsafe { tcc_relocate(self.inner, null_mut()) };
    if len == -1 {
      return Err(());
    };
    let mut bin = Vec::with_capacity(len as usize);
    let ret =
      // SAFETY: bin is allocated up to len.
      unsafe { tcc_relocate(self.inner, bin.as_mut_ptr() as *mut c_void) };
    if ret != 0 {
      return Err(());
    }
    // SAFETY: if ret == 0, bin is initialized.
    unsafe {
      bin.set_len(len as usize);
    }
    self.bin = Some(bin);
    // SAFETY: sym is a null-terminated C string.
    let addr = unsafe { tcc_get_symbol(self.inner, sym.as_ptr()) };
    Ok(addr)
  }
}