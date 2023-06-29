#![allow(unused)]
#![feature(allocator_api)]

fn main() {
    use std::alloc::System;

    let x = Box::new_in(String::from("Hello"), System);
    let (ptr, alloc) = Box::into_raw_with_allocator(x);
    let x = unsafe { Box::from_raw_in(ptr, alloc) };
}