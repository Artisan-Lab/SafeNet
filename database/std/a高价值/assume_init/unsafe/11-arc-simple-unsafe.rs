#![allow(unused)]
#![feature(new_uninit)]
#![feature(get_mut_unchecked)]
use std::sync::Arc;

fn main() {
    let mut five = Arc::<u32>::new_uninit();

    Arc::get_mut(&mut five).unwrap().write(5);

    let five = unsafe { five.assume_init() };
}