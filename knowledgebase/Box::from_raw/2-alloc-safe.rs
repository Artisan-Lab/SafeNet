/*
    Replaceable? Yes
*/
#![allow(unused)]

fn main() {
    let mut x = Box::new(1i32);
    *x = 2;
}
