#![allow(unused)]
#![feature(allocator_api)]

use std::alloc::{AllocError, Allocator, Global, Layout};

fn main() {
    let mut vec = Vec::with_capacity_in(16, Global);
    vec.push(1_000_000);

    assert_eq!(vec, &[1_000_000]);
    assert_eq!(vec.capacity(), 16);
}
