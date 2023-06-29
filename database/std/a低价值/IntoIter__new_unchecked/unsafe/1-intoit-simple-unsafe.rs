#![allow(unused)]
#![feature(array_into_iter_constructors)]
#![feature(maybe_uninit_uninit_array_transpose)]
#![feature(maybe_uninit_uninit_array)]
use std::array::IntoIter;
use std::mem::MaybeUninit;
fn main() {
    let r: [_; 4] = next_chunk(&mut (10..16)).unwrap();
    let r: IntoIter<_, 40> = next_chunk(&mut (10..16)).unwrap_err();
}

fn next_chunk<T: Copy, const N: usize>(
    it: &mut impl Iterator<Item = T>,
) -> Result<[T; N], IntoIter<T, N>> {
    let mut buffer = MaybeUninit::uninit_array();
    let mut i = 0;
    while i < N {
        match it.next() {
            Some(x) => {
                buffer[i].write(x);
                i += 1;
            }
            None => {
                // SAFETY: We've initialized the first `i` items
                unsafe {
                    return Err(IntoIter::new_unchecked(buffer, 0..i));
                }
            }
        }
    }
    unsafe { Ok(buffer.transpose().assume_init()) }
}