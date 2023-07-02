#![allow(unused)]
#![feature(vec_into_raw_parts)]

fn USE<T>(x: T) {}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let mut v1 = Vec::with_capacity(v.len());
    
    for elem in v {
        v1.push(elem);
    }

    USE(&v1 as *const _);
    USE(v1);
}
