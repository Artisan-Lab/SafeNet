#![allow(unused)]
#![feature(new_uninit)]

fn main() {
    let zero = Box::<u32>::new_zeroed();
    let zero = unsafe { zero.assume_init() };
    assert_eq!(*zero, 0);

}