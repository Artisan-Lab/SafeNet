/*
    This is a case from std
    Link: https://doc.rust-lang.org/std/string/struct.String.html#method.get_unchecked
    Purpose: Returns an unchecked subslice of str.
    Replaceable? Yes
*/

#![allow(unused)]
fn main() {
    let str = "hello";
    let a =  unsafe { str.get_unchecked(0..2) };
    assert_eq!(a, "he");
}
