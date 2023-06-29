#![allow(unused)]
#![feature(allocator_api, slice_ptr_get)]

fn main() {
    use std::alloc::{Allocator, Layout, System};

    let x = unsafe {
        let ptr = System.allocate(Layout::new::<i32>()).unwrap().as_mut_ptr() as *mut i32;
        ptr.write(5);
        Box::from_raw_in(ptr, System)
    };

}