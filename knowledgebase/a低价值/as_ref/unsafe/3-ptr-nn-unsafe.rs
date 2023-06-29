#![allow(unused)]
fn main() {
    use std::ptr::NonNull;

    let mut x = 0u32;
    let ptr = NonNull::new(&mut x as *mut _).expect("ptr is null!");

    let ref_x = unsafe { ptr.as_ref() };
}