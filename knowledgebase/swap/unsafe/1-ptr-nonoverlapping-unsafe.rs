#![allow(unused)]
use std::ptr;

fn main() {

    let mut array = [0, 1, 2, 3];

    let (x, y) = array.split_at_mut(2);
    let x = x.as_mut_ptr().cast::<[u32; 2]>(); // this is `array[0..2]`
    let y = y.as_mut_ptr().cast::<[u32; 2]>(); // this is `array[2..4]`

    unsafe {
        ptr::swap(x, y);
    }
}