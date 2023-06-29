#![allow(unused)]
#![feature(get_mut_unchecked)]

fn main() {
    use std::rc::Rc;

    let mut x = Rc::new(String::new());
    unsafe {
        //  使用 unsafe 块来使用 get_mut_unchecked 方法，强制获取 Rc 智能指针的可变引用，并调用 push_str 方法添加字符串 "foo"
        Rc::get_mut_unchecked(&mut x).push_str("foo")
    }
    assert_eq!(*x, "foo");
}