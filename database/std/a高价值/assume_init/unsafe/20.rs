#![allow(unused)]
#![feature(new_uninit)]

fn main() {
    use std::rc::Rc;

    let zero = Rc::<u32>::new_zeroed();
    let zero = unsafe { zero.assume_init() };

    assert_eq!(*zero, 0)
}