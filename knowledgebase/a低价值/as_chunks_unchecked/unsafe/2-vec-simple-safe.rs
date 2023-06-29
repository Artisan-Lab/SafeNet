#![allow(unused)]
#![feature(slice_as_chunks)]
fn main() {
    let v = vec!['l', 'o', 'r', 'e', 'm'];
    let (chunks, remainder) = v.as_chunks::<2>();
    assert_eq!(chunks, &[['l', 'o'], ['r', 'e']]);
    assert_eq!(remainder, &['m']);
}
