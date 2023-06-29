#![allow(unused)]
#![feature(allocator_api)]

fn main() {
    use std::alloc::{Allocator, Layout, System};
    use std::ptr::{self, NonNull};

    // 在 System 分配器上分配一个字符串的堆空间，将其放在 Box 中并将 Box 放入 x 中
    let x = Box::new_in(String::from("Hello"), System);
    // 将 Box 转化为裸指针和分配器
    let (ptr, alloc) = Box::into_raw_with_allocator(x);

    unsafe {
        // 析构 Box 内存中的值，但不释放分配器
        ptr::drop_in_place(ptr);

        // 创建一个非空指针
        let non_null = NonNull::new_unchecked(ptr);

        // 释放分配器中的内存
        // 需要指定释放的内存的布局，即字符串的大小和对齐方式
        alloc.deallocate(non_null.cast(), Layout::new::<String>());
    }
}



/*
调用 ptr::drop_in_place 函数，显式地销毁了 String 实例，然后使用 alloc 实例的 deallocate 方法，将分配的内存释放回去
*/