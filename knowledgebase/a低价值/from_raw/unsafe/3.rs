#![allow(unused)]
fn main() {
    use std::alloc::{alloc, Layout};

    let x = unsafe {
        let ptr = alloc(Layout::new::<i32>()) as *mut i32;
        ptr.write(5);
        Box::from_raw(ptr)
    };
    assert_eq!(*x,5)
}


// 1 2 4 5 into_raw和from_raw成对出现