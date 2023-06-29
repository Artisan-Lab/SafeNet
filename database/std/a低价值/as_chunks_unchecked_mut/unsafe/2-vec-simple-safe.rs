#![allow(unused)]
#![feature(slice_as_chunks)]
fn main() {
    let mut v = vec!['l', 'o', 'r', 'e', 'm', '!'];
    let (chunks, reminder) = v.as_chunks_mut::<3>();
    chunks[1] = ['a', 'x', '?'];
    assert_eq!(v, &['l', 'o', 'r', 'a', 'x', '?']);
}
