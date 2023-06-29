#![allow(unused)]
#![feature(downcast_unchecked)]
use std::any::Any;
use std::sync::Arc;

fn main() {
    let x: Arc<dyn Any + Send + Sync> = Arc::new(1_usize);

    unsafe {
        assert_eq!(*x.downcast_unchecked::<usize>(), 1);
    }
}