/*
    This is a simply modified case from std. The type of x in the original code is slice.
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get_unchecked_mut 
    Purpose: change the value of each Vec item
    Replaceable? Yes
*/

#![allow(unused)]
fn main() {
    let x = &mut vec![1, 2, 4];
    let elem = unsafe { x.get_unchecked_mut(1)};
    *elem = 13;
    assert_eq!(x, &[1, 13, 4]);
}
