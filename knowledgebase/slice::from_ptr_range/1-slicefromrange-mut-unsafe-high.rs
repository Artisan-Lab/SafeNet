#![allow(unused)]
#![feature(slice_from_ptr_range)]

fn main() {
    use core::slice;

    let mut x = [1, 2, 3];
    let range = x.as_mut_ptr_range();

    unsafe {
        slice::from_mut_ptr_range(range);
    }
}