pub fn xcalloc_zeroed<T: SafelyZero>(len: usize) -> Option<&'static mut [T]> {
    if len == 0 || mem::size_of::<T>() == 0 {
        return None;
    }
    // SAFETY: We're not asking for zero size because of above check
    let ptr = unsafe { xcalloc(len, mem::size_of::<T>()) };
    if ptr.is_null() {
        None
    } else {
        // SAFETY: Allocating returns a valid pointer if non-null, it's valid for len * size_of::<T>()
        //         bytes because that's what we requested
        unsafe { ptr::write_bytes(ptr, 0, len) };
        // SAFETY: Same as above, plus `SafelyZero` means it's sound to return a reference to all-zero T
        Some(unsafe { slice::from_raw_parts_mut(ptr.cast(), len) })
    }
}
// https://github.com/tectonic-typesetting/tectonic/blob/a2865a62621d1e00e748ee8c3ef203954a562e60/crates/engine_bibtex/src/c_api/xbuf.rs#L36