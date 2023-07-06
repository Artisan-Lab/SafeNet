pub fn with_len(len: usize) -> Self {
    assert!(len < std::u32::MAX as usize);
    let bytes_amount = len.checked_mul(std::mem::size_of::<T>()).expect("Attempted to allocate too large an Array");

    // WinRT arrays must be allocated with CoTaskMemAlloc.
    // SAFETY: the call to CoTaskMemAlloc is safe to perform
    // if len is zero and overflow was checked above.
    // We ensured we alloc enough space by multiplying len * size_of::<T>
    let data = unsafe { CoTaskMemAlloc(bytes_amount) as *mut T::DefaultType };

    assert!(!data.is_null(), "Could not successfully allocate for Array");

    // SAFETY: It is by definition safe to zero-initialize WinRT types.
    // `write_bytes` will write 0 to (len * size_of::<T>())
    // bytes making the entire array zero initialized. We have assured
    // above that the data ptr is not null.
    unsafe {
        std::ptr::write_bytes(data, 0, len);
    }

    let len = len as u32;
    Self { data, len }
}
// https://github.com/makepad/makepad/blob/17eef68bd1347eca1b920390642f9f44f25e4a6f/libs/windows/src/core/array.rs#L40