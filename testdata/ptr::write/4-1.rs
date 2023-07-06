unsafe fn alloc(height: usize, ref_count: usize) -> *mut Self {
    let layout = Self::get_layout(height);
    let ptr = alloc(layout).cast::<Self>();
    if ptr.is_null() {
        handle_alloc_error(layout);
    }

    ptr::write(
        &mut (*ptr).refs_and_height,
        AtomicUsize::new((height - 1) | ref_count << HEIGHT_BITS),
    );
    ptr::write_bytes((*ptr).tower.pointers.as_mut_ptr(), 0, height);
    ptr
}
// https://github.com/crossbeam-rs/crossbeam/blob/ce31c18607c44d3d07fc3618f981e858b35e3828/crossbeam-skiplist/src/base.rs#L107