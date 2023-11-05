pub unsafe fn allocate_bytes(size: usize) -> *mut u8 {
    let layout = get_usize_align_layout(size);
    let ret = alloc::alloc_zeroed(layout.clone());
    if ret.is_null() {
        writeln!(
            io::stderr(),
            "Failed to allocate memory (layout={:?})",
            layout,
        )
        .unwrap();
        abort();
    };

    ptr::write_bytes(ret, 0, size);
    ret
}
// https://github.com/Aatch/ramp/blob/9555445f0232fd558a0c5893cfa256e54a1fc896/src/mem.rs#L60