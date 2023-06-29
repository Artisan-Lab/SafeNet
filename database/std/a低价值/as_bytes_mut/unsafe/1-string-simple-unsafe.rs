/*
    This is a case from std
    Link: https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes_mut
    Purpose: Converts a mutable string slice to a mutable byte slice.
    Replaceable? Yes
*/

#![allow(unused)]

fn main() {
    let mut s = String::from("Hello");
    let bytes = unsafe { s.as_bytes_mut() };
    bytes[0] = b'h';
    assert_eq!(b"hello", bytes);
}
