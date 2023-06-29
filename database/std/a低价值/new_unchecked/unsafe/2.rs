#![allow(unused)]
fn main() {
    use std::ptr::NonNull;

// NEVER DO THAT!!! This is undefined behavior.
    let ptr = unsafe { NonNull::<u32>::new_unchecked(std::ptr::null_mut()) };
}