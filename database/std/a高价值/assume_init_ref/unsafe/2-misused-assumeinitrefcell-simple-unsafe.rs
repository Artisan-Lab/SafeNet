#![allow(unused)]
use std::{cell::Cell, mem::MaybeUninit};

fn main() {
    let b = MaybeUninit::<Cell<bool>>::uninit();
    unsafe {
        b.assume_init_ref().set(true);
    }
}