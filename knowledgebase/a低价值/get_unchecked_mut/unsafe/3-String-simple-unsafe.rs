/*
    This is a modified case from std
    Link: https://doc.rust-lang.org/std/string/struct.String.html#method.get_unchecked_mut
    Purpose: Returns an unchecked subslice of str.
    Replaceable? Yes
*/

#![allow(unused)]
fn main() {
    let mut v = String::from("hello");
    let a =  unsafe { v.get_unchecked_mut(0..2) };
    a.get_mut(0..2).map(|i| {i.make_ascii_uppercase();});
    assert_eq!(v, "HEllo");
}
