/*
https://doc.rust-lang.org/std/boxed/struct.Box.html#method.assume_init
*/

#![allow(unused)]
#![feature(new_uninit)]

fn main() {
    let mut five = Box::<u32>::new_uninit();
    let five: Box<u32> = unsafe {
        five.as_mut_ptr().write(5);
        five.assume_init()
    };
    assert_eq!(*five, 5)
}
