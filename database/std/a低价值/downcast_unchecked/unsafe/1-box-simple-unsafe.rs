#![allow(unused)]
#![feature(downcast_unchecked)]
use std::any::Any;

fn main() {
    let x: Box<dyn Any> = Box::new(1_usize);

    unsafe {
        assert_eq!(*x.downcast_unchecked::<usize>(), 1);
    }
}