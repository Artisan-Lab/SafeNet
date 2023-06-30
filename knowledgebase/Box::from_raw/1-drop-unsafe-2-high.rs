#![allow(unused)]
fn main() {
    // 创建一个堆上分配的 i32 类型的 Box，其值为 88
    let my_speed: Box<i32> = Box::new(88);
    // 把 Box 转换为指向内存地址的裸指针
    let my_speed: *mut i32 = Box::into_raw(my_speed);
    unsafe {
        // 用 drop 函数释放内存，在释放内存之前，使用 Box::from_raw 函数从裸指针重新创建 Box 类型对象
        drop(Box::from_raw(my_speed));
    }
}
