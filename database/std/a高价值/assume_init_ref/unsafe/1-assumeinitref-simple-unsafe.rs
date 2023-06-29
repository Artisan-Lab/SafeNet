#![allow(unused)]
use std::mem::MaybeUninit;

fn main() {
    let mut x = MaybeUninit::<Vec<u32>>::uninit();
    x.write(vec![1, 2, 3]);
    let x: &Vec<u32> = unsafe {
        x.assume_init_ref()
    };
}