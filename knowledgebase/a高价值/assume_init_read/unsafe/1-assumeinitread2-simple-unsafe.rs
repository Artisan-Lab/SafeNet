#![allow(unused)]
use std::mem::MaybeUninit;

fn main() {
    let mut x = MaybeUninit::<u32>::uninit();
    x.write(13);
    let x1 = unsafe { x.assume_init_read() };
    let x2 = unsafe { x.assume_init_read() };
}