#![allow(unused)]
#![feature(slice_from_ptr_range)]
use core::slice;

fn main() {
    let x = [1, 2, 3];
    let range = x.as_ptr_range();

    unsafe {
        slice::from_ptr_range(range);
    }
}