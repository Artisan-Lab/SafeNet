pub fn heap_alloc(bytes: usize) -> crate::Result<*mut std::ffi::c_void> {
    let ptr = unsafe { HeapAlloc(GetProcessHeap(), 0, bytes) };

    if ptr.is_null() {
        Err(E_OUTOFMEMORY.into())
    } else {
        // HeapAlloc is not guaranteed to return zero memory but usually does. This just ensures that
        // it predictably returns non-zero memory for testing purposes. This is similar to what MSVC's
        // debug allocator does for the same reason.
        #[cfg(debug_assertions)]
        unsafe {
            std::ptr::write_bytes(ptr, 0xCC, bytes);
        }
        Ok(ptr)
    }
}
// https://github.com/microsoft/windows-rs/blob/7b424c2590ac21b47e8ac16b31460fd625e28122/crates/libs/core/src/imp/heap.rs#L20