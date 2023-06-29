use std::mem::MaybeUninit;

fn main() {
    let mut v: [MaybeUninit<Vec<i32>>; 10] = unsafe { MaybeUninit::uninit().assume_init() };
}
