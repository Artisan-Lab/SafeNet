#![allow(unused)]
#![feature(new_uninit)]

fn main() {
    // 创建一个包含整数5的Box，Box是一个智能指针，可用于在堆上分配数据并安全地访问它们
    let mut five = Box::<u32>::new(5);
    // 使用解引用操作符*获取Box指针中存储的值，并断言其等于5
    assert_eq!(*five, 5);

}