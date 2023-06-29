#![allow(unused)]
#![feature(allocator_api, slice_ptr_get)]

fn main() {
    use std::alloc::{Allocator, Layout, System};

    let x = Box::new_in(5,System);

}