/*
https://doc.rust-lang.org/std/rc/struct.Rc.html#method.assume_init
*/

#![allow(unused)]
#![feature(new_uninit)]
#![feature(get_mut_unchecked)]
use std::rc::Rc;

fn main() {
    let mut five = Rc::<u32>::new_uninit();
    Rc::get_mut(&mut five).unwrap().write(5);
    let five = unsafe { five.assume_init() };
    assert_eq!(*five, 5)
}
