#![allow(unused)]
#![feature(downcast_unchecked)]

fn main() {
    use std::any::Any;

    let x: Box<dyn Any> = Box::new(1_usize);

    if let Ok(value) = x.downcast::<usize>() {
        assert_eq!(*value, 1);
    }

}

