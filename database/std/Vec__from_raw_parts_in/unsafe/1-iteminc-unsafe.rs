/*
    This is a case from std
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_raw_parts 
    Purpose: change the value of each Vec item
    Replaceable? Yes
*/
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

    let mut v = mem::ManuallyDrop::new(v);
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    let alloc = v.allocator();

    unsafe {
        for i in 0..len {
            ptr::write(p.add(i), 4 + i);
        }
        let rebuilt = Vec::from_raw_parts_in(p, len, cap, alloc.clone());
        assert_eq!(rebuilt, [4, 5, 6]);
    }
}
