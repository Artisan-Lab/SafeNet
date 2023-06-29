#![allow(unused)]
fn main() {
    use std::ptr::NonNull;

// NEVER DO THAT!!! This is undefined behavior.
    let ptr =  NonNull::<u32>::new(std::ptr::null_mut()).expect("ptr is null!");
}