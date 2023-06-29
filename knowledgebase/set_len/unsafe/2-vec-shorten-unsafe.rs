/*
    This is a case from std
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.set_len
    Purpose: shorten the length of a Vec while maintaining its capacity
    Replaceable? Yes
*/
#![allow(unused)]
fn main() {
    let mut vec = vec![vec![1, 0, 0],
                       vec![0, 1, 0],
                       vec![0, 0, 1]];
    unsafe {
        vec.set_len(0);
    }
}
