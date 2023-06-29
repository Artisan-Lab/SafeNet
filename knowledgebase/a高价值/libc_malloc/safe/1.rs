#![allow(unused)]
#![feature(rustc_private)]

fn main() {
    use std::mem;
    let mut y: i32 = 1;
    let my_num: *mut i32 = y as *mut i32;
    if my_num.is_null() {
        panic!("failed to allocate memory");
    }

    drop(my_num);
}