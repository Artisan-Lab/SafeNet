#![allow(unused)]
fn main() {
    let mut a = [0; 5];
    let ptr1: *mut i32 = &mut a[1];
    let ptr2: *mut i32 = &mut a[3];
    unsafe {
        ptr2.offset_from(ptr1);
    }
}