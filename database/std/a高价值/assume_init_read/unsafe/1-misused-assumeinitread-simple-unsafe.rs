#![allow(unused)]
use std::mem::MaybeUninit;

fn main() {
    let mut x = MaybeUninit::<Option<Vec<u32>>>::uninit();
    x.write(Some(vec![0, 1, 2]));
    let x1 = unsafe { x.assume_init_read() };
    let x2 = unsafe { x.assume_init_read() };
}