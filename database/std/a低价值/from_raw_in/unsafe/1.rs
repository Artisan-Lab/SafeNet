#![allow(unused)]
#![feature(allocator_api)]

use std::alloc::System;

fn main() {
    // 使用 System 分配器来创建一个 i32 类型的 Box
    let y = Box::new_in(5, System);

    // 获得指向内存地址的原始裸指针和分配器
    let (ptr, alloc) = Box::into_raw_with_allocator(y);

    // 将一个 Box 从裸指针中重建，指定相应的分配器
    let x = unsafe { Box::from_raw_in(ptr, alloc) };
}


// 当使用 System 分配器时，Rust的内存管理方式类似于C语言，需要手动管理内存的分配和释放