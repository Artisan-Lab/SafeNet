/*
    This is a simply case from std. 
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get_unchecked 
    Purpose: change the value of each Vec item
    Replaceable? Yes
*/

#![allow(unused)]
fn main() {
    let x = [1, 2, 4];
    let item = unsafe { x.get_unchecked(1) };
    assert_eq!(item, &2);
}
