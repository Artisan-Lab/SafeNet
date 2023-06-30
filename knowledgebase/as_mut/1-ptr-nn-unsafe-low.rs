#![allow(unused)]
fn main() {
    use std::ptr::NonNull;

    let mut x = 0u32;
    let mut ptr = NonNull::new(&mut x).expect("null pointer");

    let x_ref = unsafe { ptr.as_mut() };
}