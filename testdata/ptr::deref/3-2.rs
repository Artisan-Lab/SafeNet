pub extern "C" fn zfree(_ptr: *mut c_void, address: *mut c_void) {
    // SAFETY: Move our address being free'd back one pointer, read the size we
    // stored in `zalloc`, and then free it using the standard Rust
    // allocator.
    unsafe {
        let ptr = (address as *mut usize).offset(-1);
        let size = *ptr;
        let layout = Layout::from_size_align_unchecked(size, ALIGN);
        alloc::dealloc(ptr as *mut u8, layout)
    }
}
//https://github.com/suddenlyGiovanni/deno/blob/55f01508540e015563e5e54fd0652e81b347b9c1/ext/node/ops/zlib/alloc.rs#L60