fn drop(&mut self) {
    // extract the slice so we can use `Drop for [T]`
    let slice_ptr: *mut [T] = mem::take::<&'data mut [T]>(&mut self.slice);
    unsafe { ptr::drop_in_place::<[T]>(slice_ptr) };
}
// https://github.com/rayon-rs/rayon/blob/6a9deff7a99e7c7b03bab694bbe8154d6fe2de10/src/vec.rs#L230