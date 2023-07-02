#![allow(unused)]
#![feature(vec_into_raw_parts)]

fn USE<T>(x: T) {}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let (p, l, c) = v.into_raw_parts();
    USE(p);
    let v1 = unsafe { Vec::from_raw_parts(p, l, c) };
    USE(v1);
}
