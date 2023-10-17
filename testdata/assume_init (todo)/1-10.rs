fn new_buffer() -> &'static mut [MaybeUninit<V>] {
    // Create an uninitialized slice of values and leak it
    // to promote it to the static lifetime
    let buffer: [MaybeUninit<V>; INTERN_TABLE_BUFFER_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    Box::leak(Box::new(buffer))
}

// https://github.com/chrisportela/relay/blob/5a9d42e62c02ac48f37db7ad344b55c9297a6bee/compiler/crates/interner/src/generic.rs#L89