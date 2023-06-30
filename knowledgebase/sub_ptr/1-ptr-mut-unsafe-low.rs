#![allow(unused)]
#![feature(ptr_sub_ptr)]

fn main() {
    let mut a = [0; 5];
    let p: *mut i32 = a.as_mut_ptr();
    unsafe {
        let ptr1: *mut i32 = p.add(1);
        let ptr2: *mut i32 = p.add(3);
        ptr2.sub_ptr(ptr1);
    }
}