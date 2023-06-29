/*
    This is a slightly modified case from std
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get_many_unchecked_mut
    Purpose: Returns mutable references to many indices at once, without doing any checks.
    Replaceable? Yes
*/

#![allow(unused)]
#![feature(get_many_mut)]

fn main() {
    let mut v = vec![1, 2, 3, 4];
    let [a, b, d] = unsafe { v.get_many_unchecked_mut([0, 1, 3]) };
    *a = 10;
    *b = 100;
    assert_eq!(v, &[10, 100, 3, 4]);
}
