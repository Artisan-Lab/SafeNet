#![allow(unused)]
fn main() {
    let mut s = [1, 2, 3];
    let ptr: *mut u32 = s.as_mut_ptr();

    unsafe {
        println!("{}", *ptr.offset(1));
        println!("{}", *ptr.offset(2));
    }
}