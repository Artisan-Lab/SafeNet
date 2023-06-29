#![allow(unused)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
use std::mem::MaybeUninit;

fn main() {
    let mut array: [MaybeUninit<i32>; 3] = MaybeUninit::uninit_array();
    array[0].write(0);

    let array = unsafe {
        MaybeUninit::array_assume_init(array)
    };

}