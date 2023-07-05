pub fn allocate_zeroed(layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    let layout_size = layout.size();
    if layout.align() <= MIN_ALIGN && layout.align() <= layout_size {
        NonNull::new(unsafe { libc::calloc(layout_size, 1) as *mut u8 })
            .ok_or(AllocError)
            .map(|ptr| NonNull::slice_from_raw_parts(ptr, layout_size))
    } else {
        let ptr = allocate(layout)?;
        unsafe {
            ptr::write_bytes(ptr.as_mut_ptr(), 0, layout_size);
        }
        Ok(ptr)
    }
}
// https://github.com/GetFirefly/firefly/blob/8e89bc7ec33cb8ffa9a60283c8dcb7ff62ead5fa/library/system/src/unix/alloc.rs#L35