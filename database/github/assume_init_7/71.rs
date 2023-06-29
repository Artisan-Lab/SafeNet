use std::mem::{self, MaybeUninit};
fn new_buffer() -> &'static mut [MaybeUninit<V>] {
    // Create an uninitialized slice of values and leak it
    // to promote it to the static lifetime
    let buffer: [MaybeUninit<V>; INTERN_TABLE_BUFFER_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    Box::leak(Box::new(buffer))
}
/*
https://github.com/vincentriemer/relay/blob/b6c6b7d28a0121f97ad20d17c716c3b905ee6c82/compiler/crates/interner/src/generic.rs#L89
*/