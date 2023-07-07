/*
    This is a case from std
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_raw_parts 
    Purpose: create a vec from a memory with a specific capacity 
    Replaceable? Yes
*/

#![feature(vec_into_raw_parts)]
fn convert_to_i8(input: Vec<u8>) -> Vec<i8> {
    let (p, l, c) = input.into_raw_parts();
    unsafe { Vec::from_raw_parts(p as *mut i8, l, c) }
}


fn main() {
    let mut v: Vec<u8> = vec![1;10];
    convert_to_i8(v);
}