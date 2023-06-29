#![allow(unused)]
use std::rc::Rc;
use std::ops::Deref;
use core::mem::size_of;
fn main() {
    let a = [0; 5];

    let ptr1: *const i32 = &a[1];
    let ptr2: *const i32 = &a[3];
    let diff1:isize = (ptr2 as isize).wrapping_sub(ptr1 as isize);
    let diff2:isize = (ptr1 as isize).wrapping_sub(ptr2 as isize);
    let size_of_i32 = size_of::<i32>() as isize;
    assert_eq!(diff1/size_of_i32, 2);
    assert_eq!(diff2/size_of_i32, -2);


}