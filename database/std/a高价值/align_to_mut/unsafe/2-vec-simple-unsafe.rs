/*
    This is a slightly modified case from std
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.align_to_mut
    Purpose: Transmute the slice to a slice of another type, ensuring alignment of the types is maintained.
    Replaceable? Yes?
*/

#![allow(unused)]
fn main() {
    let mut v = vec![1u8;7];
    let (prefix, shorts, suffix) = unsafe { v.align_to_mut::<u16>()};
    shorts[0] = 100;
    assert_eq!(prefix, &[]);
    assert_eq!(shorts, &[100, 257, 257]);
    assert_eq!(suffix, &[1]);
}
