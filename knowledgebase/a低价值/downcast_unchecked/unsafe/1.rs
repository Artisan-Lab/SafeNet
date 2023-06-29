#![allow(unused)]
#![feature(downcast_unchecked)]

fn main() {
    use std::any::Any;

    // 创建了一个 Box 指针，指向一个 usize 类型的整数 1。
    // 这个 Box 指针的类型是 Box<dyn Any>，它可以持有任何实现了 Any trait 的类型的值。
    let x: Box<dyn Any> = Box::new(1_usize);
    // trait 对象的类型是不确定的，因此需要使用 dyn 关键字来表示

    unsafe {
        // 调用 downcast_unchecked 方法，将 Box<dyn Any> 类型的值转换为 usize 类型的值。
        // 这个方法的返回值是一个裸指针 *mut T，它指向了 Box 中持有的实际值的内存位置。
        assert_eq!(*x.downcast_unchecked::<usize>(), 1);
    }
}

