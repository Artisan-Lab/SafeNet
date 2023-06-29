#![allow(unused)]
use std::ops::Deref;
fn main() {
    use std::rc::Rc;

    let x = Rc::new("hello".to_owned());

    assert_eq!( &*x , "hello");
    // assert_eq!(x.deref(), "hello");
    // x.deref() 和 &*x 都是将 Rc<String> 转换为 &str 类型的引用
    // 原unsafe代码此处为&*x 不需要修改 

}
