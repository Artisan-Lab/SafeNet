#![allow(unused)]
use std::mem::MaybeUninit;
fn main() {
    let mut m: [MaybeUninit<String>; 256] = unsafe { MaybeUninit::uninit().assume_init() };
}