#![allow(unused)]
#![feature(get_mut_unchecked)]

fn main() {
    use std::sync::Arc;

    let x: Arc<&str> = Arc::new("Hello, world!");
    {
        let s = String::from("Oh, no!");
        let mut y: Arc<&str> = x.clone().into();
        unsafe {
            *Arc::get_mut_unchecked(&mut y) = &s;
        }
    }
}