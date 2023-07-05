fn drop(&mut self) {
    let layout = ComponentInstance::alloc_layout(&self.offsets);
    unsafe {
        ptr::drop_in_place(self.ptr.as_ptr());
        alloc::dealloc(self.ptr.as_ptr().cast(), layout);
    }
}
// https://github.com/bytecodealliance/wasmtime/blob/860d46b4d149820e6216229fd1def4214b3c50e5/crates/runtime/src/component.rs#L642