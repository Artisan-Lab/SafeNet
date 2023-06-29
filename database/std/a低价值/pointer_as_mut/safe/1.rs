#![allow(unused)]
fn main() {
    let mut s = [1, 2, 3];
    s[0]=4;
    assert_eq!(s, [4, 2, 3]);
    println!("{:?}", s); // It'll print: "[4, 2, 3]".
}