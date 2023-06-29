#![allow(unused)]
fn main() {
    let mut s = [1, 2, 3];
    let ptr: *mut u32 = s.as_mut_ptr();
    let first_value = unsafe { ptr.as_mut().unwrap() };
    *first_value = 4;
    assert_eq!(s, [4, 2, 3]);
    println!("{:?}", s); // It'll print: "[4, 2, 3]".
}