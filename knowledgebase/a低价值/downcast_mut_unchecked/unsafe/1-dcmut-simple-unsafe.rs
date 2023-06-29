#![allow(unused)]
#![feature(downcast_unchecked)]

fn main() {
    use std::any::Any;

    let mut x: Box<dyn Any> = Box::new(1_usize);

    unsafe {
        *x.downcast_mut_unchecked::<usize>() += 1;
    }

}