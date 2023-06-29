#![allow(unused)]
use std::rc::Rc;
use std::ops::Deref;
use core::mem::size_of;
fn main() {
    let mut a = [0; 5];
    let ptr1: *mut i32 = &mut a[1];
    let ptr2: *mut i32 = &mut a[3];

    assert_eq!(ptr1.wrapping_offset(2), ptr2);
    assert_eq!(ptr2.wrapping_offset(-2), ptr1);


}