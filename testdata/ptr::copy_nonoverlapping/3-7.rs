fn from(slice: &[T]) -> Box<[T]> {
    let len = slice.len();
    let buf = RawVec::with_capacity(len);
    unsafe {
        ptr::copy_nonoverlapping(slice.as_ptr(), buf.ptr(), len);
        buf.into_box(slice.len()).assume_init()
    }
}
// https://github.com/beagleboard/linux/blob/98be618ad03010b1173fc3c35f6cbb4447ee2b07/rust/alloc/boxed.rs#L1505