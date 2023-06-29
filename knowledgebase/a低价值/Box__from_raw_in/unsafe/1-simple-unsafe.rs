#![allow(unused)]
#![feature(allocator_api, slice_ptr_get)]
use std::alloc::{Allocator, Layout, System};

fn foo() -> Result<(), impl core::fmt::Debug> {
    let ptr = unsafe { 
        System.allocate(Layout::new::<i32>())?.as_mut_ptr() as *mut i32
    };
    unsafe { ptr.write(0); }
    let mut x = unsafe { Box::from_raw(ptr) };
    *x = 2;
    Ok::<(), std::alloc::AllocError>(())
}

fn main() {
    foo().unwrap();
}
