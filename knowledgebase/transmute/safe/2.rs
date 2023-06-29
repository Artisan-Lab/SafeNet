#![allow(unused)]
fn main() {
    let ptr = &0;
    let ptr_num = ptr as *const i32 as usize;
}