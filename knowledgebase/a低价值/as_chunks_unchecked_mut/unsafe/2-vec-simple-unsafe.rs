/*
    This is a modified case from std
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_chunks_unchecked_mut
    Purpose: Splits the slice into a slice of N-element arrays, assuming that there’s no remainder.
    Replaceable? Yes
*/

#![allow(unused)]
#![feature(slice_as_chunks)]
fn main() {
    let mut v = vec!['l', 'o', 'r', 'e', 'm', '!'];
    let chunks: &mut [[char; 3]] = unsafe { v.as_chunks_unchecked_mut() };
    chunks[1] = ['a', 'x', '?'];
    assert_eq!(v, &['l', 'o', 'r', 'a', 'x', '?']);
}
