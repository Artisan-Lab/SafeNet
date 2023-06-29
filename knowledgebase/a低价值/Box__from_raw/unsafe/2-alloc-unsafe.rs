
#![allow(unused)]
use std::alloc::{alloc, Layout};

fn main() {
    let ptr = unsafe { 
        alloc(Layout::new::<i32>()) as *mut i32
    };
    unsafe { ptr.write(0); }
    let mut x = unsafe { Box::from_raw(ptr) };
    *x = 2;
}
