/*
From: https://doc.rust-lang.org/std/mem/fn.transmute.html
*/

#![allow(unused)]
fn main() {
    let ptr = &0;
    let ptr_num = unsafe {
        std::mem::transmute::<&i32, usize>(ptr)
    };
}
