#![allow(unused)]
fn main() {
    // 创建一个可变的 String 类型变量 s 并将其初始化为 "Hello"
    let mut s = String::from("Hello");
    // 使用 unsafe 关键字调用 String 类型的 as_bytes_mut() 方法，并将其返回的可变字节数组绑定到 bytes 变量上
    let bytes = unsafe { s.as_bytes_mut() };
    // 使用 assert_eq! 宏断言 bytes 数组的值等于 b"Hello"
    assert_eq!(b"Hello", bytes);
}
