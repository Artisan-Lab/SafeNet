#![allow(unused)]
use std::mem::MaybeUninit;

fn main() {
    let mut data: [MaybeUninit<String>; 1000] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut data_len: usize = 0;
    for elem in &mut data[0..500] {
        elem.write(String::from("hello"));
        data_len += 1;
    }
    
    // For each item in the array, drop if we allocated it.
    for elem in &mut data[0..data_len] {
        unsafe { elem.assume_init_drop(); }
    }
}
