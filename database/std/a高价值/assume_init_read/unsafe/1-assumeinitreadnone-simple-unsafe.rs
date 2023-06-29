#![allow(unused)]
use std::mem::MaybeUninit;

fn main() {
    let mut x = MaybeUninit::<Option<Vec<u32>>>::uninit();
    x.write(None);
    let x1 = unsafe { x.assume_init_read() };
}