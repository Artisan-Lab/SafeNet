#![allow(unused)]
use std::mem::MaybeUninit;

fn main() {
    let x = MaybeUninit::<Vec<u32>>::uninit();
    let x_vec: &Vec<u32> = unsafe { x.assume_init_ref() };
}