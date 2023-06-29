#![allow(unused)]
fn main() {
    let ptr = &0;
    let ptr = ptr as *const i32 as usize;
}
