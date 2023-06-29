#![allow(unused)]
fn main() {
    use std::rc::Rc;

    let five = Rc::new(5);

    unsafe {
        let ptr = Rc::into_raw(five);
        Rc::increment_strong_count(ptr);
    }
}