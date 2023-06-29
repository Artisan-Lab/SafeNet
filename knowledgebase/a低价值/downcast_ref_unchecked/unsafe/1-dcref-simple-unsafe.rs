#![allow(unused)]
#![feature(downcast_unchecked)]
use std::any::Any;

fn main() {

    let x: Box<dyn Any> = Box::new(1_usize);

    unsafe {
        *x.downcast_ref_unchecked::<usize>();
    }
}