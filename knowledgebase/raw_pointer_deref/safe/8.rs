#![allow(unused)]
fn main() {
    use std::cell::UnsafeCell;

    let mut x: UnsafeCell<i32> = 5.into();
// let mut x: UnsafeCell<i32> = UnsafeCell::new(5);
    *x.get_mut() += 27;


    println!("{:?}", x.get_mut());
}