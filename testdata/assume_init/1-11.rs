fn zeroed(size: Size, _align: Align) -> Option<Self> {
    let bytes = Box::<[u8]>::try_new_zeroed_slice(size.bytes_usize()).ok()?;
    // SAFETY: the box was zero-allocated, which is a valid initial value for Box<[u8]>
    let bytes = unsafe { bytes.assume_init() };
    Some(bytes)
}