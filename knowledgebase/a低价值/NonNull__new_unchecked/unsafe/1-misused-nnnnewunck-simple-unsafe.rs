#![allow(unused)]
use std::ptr::NonNull;

fn main() {
    let ptr = unsafe { NonNull::<u32>::new_unchecked(std::ptr::null_mut()) };
}