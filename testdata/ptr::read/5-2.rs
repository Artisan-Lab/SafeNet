fn into_inner(self) -> Box<[u8]> {
    // Rationale: `mem::forget(self)` invalidates the previous call to `ptr::read(&self.inner)`
    // so we use `ManuallyDrop` to ensure `self` is not dropped.
    // Then we can return the box directly without invalidating it.
    // See https://github.com/rust-lang/rust/issues/62553.
    let this = mem::ManuallyDrop::new(self);
    unsafe { ptr::read(&this.inner) }
}
// https://github.com/rust-lang/rust/blob/99f7d368c0ed753db797ee82e89b5a2b7e49509a/library/alloc/src/ffi/c_str.rs#L605