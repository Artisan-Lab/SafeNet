#![allow(unused)]
fn main() {
    let ptr = Box::into_raw(Box::new(5));
    let x = unsafe { Box::from_raw(ptr) };
    assert_eq!(*x,5)
}
