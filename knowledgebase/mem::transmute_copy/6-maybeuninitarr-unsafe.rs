use std::mem;
use std::mem::MaybeUninit;

fn main() {
    let mut array: [MaybeUninit<i32>; 1000] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut reference: &mut [i32] = unsafe { mem::transmute_copy(&mut array) };

    for (i, elem) in reference.iter_mut().enumerate() {
        *elem = i as i32;
    }
    // for (i, elem) in reference.iter().enumerate() {
    //     println!("array[{}] = {}", i, elem);
    // }
}
