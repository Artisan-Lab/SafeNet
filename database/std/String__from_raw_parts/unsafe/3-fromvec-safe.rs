#![allow(unused)]

use std::mem;

fn main() {
    let v = vec!['h','e','l','l','o'];
    let mut s = String::new();
    let l = v.len();
    for i in 0..l {
        s.insert(i, v[i] as char);
    }
    assert_eq!(String::from("hello"), s);
}
