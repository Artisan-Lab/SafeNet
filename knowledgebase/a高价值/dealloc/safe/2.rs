#![allow(unused)]
#![feature(allocator_api)]

fn main() {
    use std::alloc::{Allocator, Layout, System};
    use std::ptr::{self, NonNull};

    let x = Box::new_in(String::from("Hello"), System);
    let (ptr, alloc) = Box::into_raw_with_allocator(x);
    drop(ptr);
    drop(alloc);
}