/// Panics if you try to increment past the end of the slice.
pub(crate) fn increment_by_mut<T>(slice: &mut &mut [T], amount: usize) {

    *slice = &mut core::mem::replace(slice, &mut [])[amount..]
}