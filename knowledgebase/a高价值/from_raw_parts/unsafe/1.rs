#![allow(unused)]
fn main() {
    use std::slice;

    let x = 42;
    let ptr = &x as *const _;
    // 创建一个指向 x 的指针 ptr
    let slice = unsafe { slice::from_raw_parts(ptr, 1) };
    // 从 ptr 创建一个只包含 x 的 slice     from_raw_parts 函数将裸指针转换成 slice 

    assert_eq!(slice[0], 42);

}