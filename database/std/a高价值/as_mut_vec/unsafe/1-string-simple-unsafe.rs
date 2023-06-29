/*
    This is a simply modified case from std
    Link: https://doc.rust-lang.org/std/string/struct.String.html#method.as_mut_vec
    Purpose: Returns a mutable reference to the contents of this String.
    Replaceable? Yes
*/

#![allow(unused)]
fn main() {
    let mut s = String::from("hello");
    let v = unsafe { s.as_mut_vec() };
    assert_eq!(v, &vec![b'h',b'e',b'l',b'l',b'o']);
    v.reverse();
    assert_eq!(s, "olleh");
}
