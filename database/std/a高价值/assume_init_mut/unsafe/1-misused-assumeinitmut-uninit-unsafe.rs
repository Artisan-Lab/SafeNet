#![allow(unused)]
use std::mem::MaybeUninit;

fn main() {
    let mut b = MaybeUninit::<bool>::uninit();
    unsafe {
        *b.assume_init_mut() = true;
    }
}