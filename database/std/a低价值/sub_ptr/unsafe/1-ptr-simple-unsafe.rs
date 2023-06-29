#![allow(unused)]
#![feature(ptr_sub_ptr)]

fn main() {
    let a = [0; 5];
    let ptr1: *const i32 = &a[1];
    let ptr2: *const i32 = &a[3];
    unsafe {
        ptr2.sub_ptr(ptr1);
    }
}