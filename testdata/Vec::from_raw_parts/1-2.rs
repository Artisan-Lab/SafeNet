fn from(data: Vec<T>) -> Self {
    unsafe fn drop_vec<T>(ptr: *mut u8, len: usize, cap: usize) {
        let _ = Vec::from_raw_parts(ptr as *mut T, len, cap);
    }

    // FIXME(adamreichold): Use `Vec::into_raw_parts`
    // when it becomes stable and compatible with our MSRV.
    let mut data = mem::ManuallyDrop::new(data);

    let ptr = data.as_mut_ptr() as *mut u8;
    let len = data.len();
    let cap = data.capacity();
    let drop = drop_vec::<T>;

    Self {
        ptr,
        len,
        cap,
        drop,
    }
}

// https://github.com/PyO3/rust-numpy/blob/08510a3aa31a09c45df4602c484ac81056f0c4e8/src/slice_container.rs#L45