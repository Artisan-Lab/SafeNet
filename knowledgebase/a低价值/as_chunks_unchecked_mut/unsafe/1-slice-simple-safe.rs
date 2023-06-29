#![allow(unused)]
#![feature(slice_as_chunks)]
fn main() {
    let slice = &mut ['l', 'o', 'r', 'e', 'm', '!'];
    let (chunks, reminder) = slice.as_chunks_mut::<3>();
    chunks[1] = ['a', 'x', '?'];
    assert_eq!(slice, &['l', 'o', 'r', 'a', 'x', '?']);
}
