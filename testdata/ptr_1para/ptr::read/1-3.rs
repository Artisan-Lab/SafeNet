unsafe fn call_raw_box<F: FnOnce()>(raw: *mut u8) {
    // It's safe to cast `raw` from `*mut u8` to `*mut Box<F>`, because `raw` is
    // originally derived from `*mut Box<F>`.
    #[allow(clippy::cast_ptr_alignment)]
    let b: Box<F> = ptr::read(raw as *mut Box<F>);
    (*b)();
}

// https://github.com/spacejam/sled/blob/005c023ca94d424d8e630125e4c21320ed160031/src/ebr/deferred.rs#L31