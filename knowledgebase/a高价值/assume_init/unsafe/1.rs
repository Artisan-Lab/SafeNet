#![allow(unused)]
#![feature(new_uninit)]

fn main() {
    // 创建一个未初始化的Box<u32>类型变量
    let mut five = Box::<u32>::new_uninit();
    let five = unsafe {
        // 延迟初始化
        // 获取five的可变原始指针，并对其指向的内存赋值为5
        five.as_mut_ptr().write(5);
        // 使用assume_init()方法将MaybeUninit<Box<u32>>类型的值转换为Box<u32>类型的值
        five.assume_init()
    };
    // 确保初始化后变量的值为5
    assert_eq!(*five, 5);
}
//1 3 15 Box new_uninit直接new不再延迟初始化 5 6 +system
/*
因为Box<u32>类型的值是由系统分配的堆内存所组成，
而new_uninit()方法只是分配了这部分内存，但并未对其进行初始化，
因此需要使用write()方法将5赋值给堆内存中的u32变量，
然后使用assume_init()方法将MaybeUninit<Box<u32>>类型的值转化为Box<u32>类型的值，
确保访问到已初始化的内存，从而遵循Rust的内存安全规则

Box::new_uninit() 函数创建一个未初始化的 Box<T>，因为它是未初始化的，它的值是未定义的。
在这种情况下，使用 Box::new_uninit() 是安全的，因为它返回的 Box 还没有被视为被初始化的，
并且只有在在调用 assume_init() 前对其进行初始化才会使得 Box 的内部的值在 Rust 的内存安全性规则下定义。
使用 as_mut_ptr() 方法获取 Box 中存储值的指针，并使用 write() 方法对指针所指向的内存区域进行写入操作，
将值 5 写入内存。这个操作是不安全的，因为在 Box 中分配的内存可能还未被初始化，
直接对其进行写入操作可能导致访问未定义的内存区域。

最后，调用 assume_init() 方法将未初始化的 Box 转化为初始化的 Box。
这个操作也是不安全的，因为只有当我们确信 Box 中存储的值已经被正确初始化后，才能调用这个方法。
*/