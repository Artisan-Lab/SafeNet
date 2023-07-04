#![allow(unused)]

use std::mem;

fn foo(v: &mut mem::ManuallyDrop<Vec<u8>>) -> (*mut u8, usize, usize) {
    let ptr = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    (ptr, len, cap)
}

fn main() {
    let v = vec![1, 2, 3];
    let mut v = mem::ManuallyDrop::new(v);
    let (ptr, len, cap) = foo(&mut v);
    let vec = unsafe { Vec::from_raw_parts(ptr, len, cap) };
    assert_eq!(vec, vec![1, 2, 3]);
}
