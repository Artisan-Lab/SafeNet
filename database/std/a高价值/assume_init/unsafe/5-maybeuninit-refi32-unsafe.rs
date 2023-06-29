#![allow(unused)]
use std::mem::MaybeUninit;
fn main() {
    let mut x = MaybeUninit::<&i32>::uninit();
    x.write(&0);
    let x = unsafe { x.assume_init() };
}
