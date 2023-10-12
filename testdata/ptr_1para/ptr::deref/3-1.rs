pub fn get_wait_handles_mut(&mut self) -> &mut WaitHandleStore {
    let ptr = self.get_wait_handles_void() as *mut Box<WaitHandleStoreFFI>;
    assert!(!ptr.is_null());
    unsafe { (*ptr).from_ffi_mut() }
}
//https://github.com/fish-shell/fish-shell/blob/e3e7ab77ade306db8449fa0467891c3d4b1b9a52/fish-rust/src/ffi.rs#L165