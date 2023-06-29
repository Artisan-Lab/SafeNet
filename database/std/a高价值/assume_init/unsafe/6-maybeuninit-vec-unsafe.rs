#![allow(unused)]
use std::mem::MaybeUninit;

unsafe fn make_vec(out: *mut Vec<i32>) {
    out.write(vec![1, 2, 3]);
}
    
fn main() {
    let mut v:MaybeUninit<Vec<i32>> = MaybeUninit::uninit();
    unsafe { 
        make_vec(v.as_mut_ptr());
    }
    let v = unsafe { v.assume_init() };
    assert_eq!(&v, &[1, 2, 3]);
}
