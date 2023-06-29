/*
    This is a simply case from std. 
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get_unchecked_mut 
    Purpose: change the value of an item in the vec
    Replaceable? Yes
*/

#![allow(unused)]
fn main() {
    let x = &mut [1, 2, 4];
    let elem = unsafe { x.get_unchecked_mut(1)};
    *elem = 13;
    assert_eq!(x, &[1, 13, 4]);
}
