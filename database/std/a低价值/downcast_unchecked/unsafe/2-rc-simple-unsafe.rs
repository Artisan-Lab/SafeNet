#![allow(unused)]
#![feature(downcast_unchecked)]
use std::any::Any;
use std::rc::Rc;

fn main() {
    let x: Rc<dyn Any> = Rc::new(1_usize);

    unsafe {
        assert_eq!(*x.downcast_unchecked::<usize>(), 1);
    }
}