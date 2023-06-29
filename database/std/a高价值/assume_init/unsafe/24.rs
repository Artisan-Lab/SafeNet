#![allow(unused)]
#![feature(new_uninit)]

fn main() {
    use std::rc::Rc;

    let values = Rc::<[u32]>::new_zeroed_slice(3);
    let values = unsafe { values.assume_init() };

    assert_eq!(*values, [0, 0, 0])
}