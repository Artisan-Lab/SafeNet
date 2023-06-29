// 允许未使用的代码和使用 unstable 的 rustc 特性
#![allow(unused)]
#![feature(rustc_private)]

// 导入 libc 和 mem 模块
extern crate libc;
use std::mem;

fn main() {
    unsafe {
        // 申请分配一块大小为 i32 类型的内存块，返回值为 *mut i32 类型的指针
        let my_num: *mut i32 = libc::malloc(mem::size_of::<i32>()) as *mut i32;
        // 判断申请的内存块是否为 null 指针，如果是，抛出错误信息
        if my_num.is_null() {
            panic!("failed to allocate memory");
        }
        // 将申请到的内存块赋值为 1
        *my_num = 1;
        // 释放申请的内存块，由于类型为 i32，需要将其转为 *mut libc::c_void 类型
        libc::free(my_num as *mut libc::c_void);
    }
}
