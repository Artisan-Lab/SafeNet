/*
    This is a case from std
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_raw_parts 
    Purpose: change the value of each Vec item
    Replaceable? Yes
*/

#![allow(unused)]

use std::ptr;
use std::mem;

fn main() {
    let v = vec![1, 2, 3];
    let mut v = mem::ManuallyDrop::new(v);
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe {
        for i in 0..len {
            ptr::write(p.add(i), 4 + i);
        }
        let rebuilt = Vec::from_raw_parts(p, len, cap);
        assert_eq!(rebuilt, [4, 5, 6]);
    }
}
