/*
    This is a case from std
    Link: https://doc.rust-lang.org/std/string/struct.String.html#method.slice_unchecked
    Purpose: Creates a string slice from another string slice, bypassing safety checks.
    Replaceable? Yes
*/

#![allow(unused)]
fn main() {
    let s = "hello";
    let a = unsafe { s.slice_unchecked(0, 3) };
    assert_eq!(a, "hel");
}
