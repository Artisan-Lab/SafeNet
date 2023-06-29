#![allow(unused)]
fn main() {
    let a = [0; 5];
    let ptr1: *const i32 = &a[1];
    let ptr2: *const i32 = &a[3];
    unsafe {
        ptr2.offset_from(ptr1);
    }
}