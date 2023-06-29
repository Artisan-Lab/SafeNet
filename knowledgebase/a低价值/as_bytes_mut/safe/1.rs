#![allow(unused)]
fn main() {
    let mut s = String::from("Hello");
    let bytes = s.as_bytes() ;

    assert_eq!(b"Hello", bytes);
}

/*
as_bytes_mut 方法返回的是字符串的底层字节数组的可变引用
safe 的方式可以使用 as_bytes 方法来获取不可变的字节数组引用
*/