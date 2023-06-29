#![allow(unused)]
#![feature(new_uninit)]

fn main() {
    let zero = Box::<u32>::new(0);

    assert_eq!(*zero, 0);

}