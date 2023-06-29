/// Panics if you try to increment past the end of the slice.
pub(crate) fn increment_by_mut<T>(slice: &mut &mut [T], amount: usize) {
    let lifetime_hack = unsafe {
        let slice_ptr = slice.as_mut_ptr();
        ::std::slice::from_raw_parts_mut(slice_ptr, slice.len())
    };
    *slice = &mut lifetime_hack[amount..]

}