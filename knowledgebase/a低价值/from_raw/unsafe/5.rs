#![allow(unused)]
fn main() {
    use std::rc::Rc;

    let x = Rc::new("hello".to_owned());
    //to_owned() 方法会将传入的 &str 转换成 String 类型的字符串
    let x_ptr = Rc::into_raw(x);

    unsafe {
        let x = Rc::from_raw(x_ptr);
        assert_eq!(&*x, "hello");
    }
}