#![allow(unused)]
fn main() {
    // 导入NonNull类型
    use std::ptr::NonNull;
    // 创建一个u32类型变量x并初始化为0
    let mut x = 0u32;
    // 将x的可变指针转化为NonNull类型的指针，并检查是否为null
    let ptr = NonNull::new(&mut x as *mut _).expect("ptr is null!");
    // 使用as_ref()方法将NonNull类型的指针转化为引用类型，并打印引用的值
    let ref_x = unsafe { ptr.as_ref() };
    println!("{}", ref_x);
}
