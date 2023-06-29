#![allow(unused)]
#![feature(downcast_unchecked)]

fn main() {
    use std::any::Any;
    // dyn Any + Send 表示 x 可以持有实现了 Any trait 并且可以安全地在多线程中传递的类型的值。
    let x: Box<dyn Any+ Send> = Box::new(1_usize);
    unsafe {
        assert_eq!(*x.downcast_unchecked::<usize>(), 1);
    }
}