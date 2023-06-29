#![allow(unused)]
#![feature(slice_as_chunks)]
fn main() {
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let (chunks, remainder) = slice.as_chunks::<2>();
    assert_eq!(chunks, &[['l', 'o'], ['r', 'e']]);
    assert_eq!(remainder, &['m']);
}
