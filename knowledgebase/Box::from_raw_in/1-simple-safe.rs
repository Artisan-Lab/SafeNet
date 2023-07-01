#![allow(unused)]
#![feature(allocator_api, slice_ptr_get)]
use std::alloc::{Allocator, Layout, System};

fn foo() -> Result<(), impl core::fmt::Debug> {
    let mut x = Box::new_in(1,System);
    *x = 2;
    Ok::<(), std::alloc::AllocError>(())
}

fn main() {
    foo().unwrap();
}
