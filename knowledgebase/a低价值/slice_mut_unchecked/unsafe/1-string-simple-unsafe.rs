/*
    This is a case from std
    Link: https://doc.rust-lang.org/std/string/struct.String.html#method.slice_mut_unchecked
    Purpose: Creates a string slice from another string slice, bypassing safety checks.
    Replaceable? Yes
*/

#![allow(unused)]
fn main() {
    let mut s = String::from("hello");
    let a = unsafe { s.slice_mut_unchecked(0, 2) };
    a.get_mut(0..2).map(|i| {i.make_ascii_uppercase();});
    assert_eq!(s, "HEllo");
}
