#![allow(unused)]
fn main() {
    use std::ptr::NonNull;

    let mut x = 0u32;

    let ptr = NonNull::<u32>::new(&mut x as *mut _).expect("ptr is null!");
}