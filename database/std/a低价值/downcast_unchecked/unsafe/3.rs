#![allow(unused)]
#![feature(downcast_unchecked)]

fn main() {
    use std::any::Any;
    // Send 和 Sync 是用于表示类型是否在并发环境中安全使用的 trait。如果一个类型实现了 Send trait，则它可以在多个线程之间传递，如果一个类型实现了 Sync trait，则它可以安全地在多个线程之间共享
    let x: Box<dyn Any+ Send+ Sync> = Box::new(1_usize);

    unsafe {
        assert_eq!(*x.downcast_unchecked::<usize>(), 1);
    }
}