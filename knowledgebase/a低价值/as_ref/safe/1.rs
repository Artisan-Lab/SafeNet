#![allow(unused)]
use std::rc::Rc;
use std::ops::Deref; 

fn main() {
    // 创建一个指向u8类型变量10的不可变指针，并将其封装到一个引用计数智能指针中
    let ptr = Rc::new(10u8 as *const u8);
    // 通过Deref trait获取指针所指向的值
    let val_back = ptr.deref();
    println!("We got back the value: {:?}!", val_back);
}
