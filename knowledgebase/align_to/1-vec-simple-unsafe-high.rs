/*
    This is a slightly modified case from std
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.align_to
    Purpose: Transmute the slice to a slice of another type, ensuring alignment of the types is maintained.
    Replaceable? Yes?
*/

#![allow(unused)]
fn main() {
    let v = vec![1u8;7];
    let (prefix, shorts, suffix) = unsafe { v.align_to::<u16>()};
    assert_eq!(prefix, &[]);
    assert_eq!(shorts, &[257, 257, 257]);
    assert_eq!(suffix, &[1]);
}
