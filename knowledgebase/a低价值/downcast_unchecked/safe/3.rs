#![allow(unused)]
#![feature(downcast_unchecked)]

fn main() {
    use std::any::Any;

    let x: Box<dyn Any+ Send+ Sync> = Box::new(1_usize);

    unsafe {
        assert_eq!(*x.downcast_unchecked::<usize>(), 1);
    }
}