#![allow(unused)]
#![feature(allocator_api)]
use std::alloc::System;
use std::ptr;
use std::mem;

fn main() {
    let mut v = Vec::with_capacity_in(3, System);
    v.push(1);
    v.push(2);
    v.push(3);

    let len = v.len();
    for i in 0..len {
           v[i] = i + 4;
    }
    assert_eq!(v, [4, 5, 6]);
}
