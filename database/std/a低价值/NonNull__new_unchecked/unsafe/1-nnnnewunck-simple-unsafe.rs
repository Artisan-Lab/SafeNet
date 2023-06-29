#![allow(unused)]
use std::ptr::NonNull;

fn main() {
    let mut x = 0u32;
    let ptr = unsafe { NonNull::new_unchecked(&mut x as *mut _) };
}