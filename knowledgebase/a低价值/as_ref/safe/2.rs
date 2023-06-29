#![allow(unused)]
use std::rc::Rc;
use std::ops::Deref;

fn main() {
    // 创建一个u32类型的变量
    let mut x = 0u32;
    // 将该变量包装成一个引用计数智能指针
    let ptr = Rc::new(x);
    // 使用deref()方法获取指针内部数据的引用
    let ref_x = ptr.deref();
    // 打印引用指向的数据
    println!("{}", ref_x);
}
