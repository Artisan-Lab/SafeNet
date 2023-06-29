#![allow(unused)]
use std::str;

fn main() {

    let mut heart = vec![240, 159, 146, 150];
    let heart = unsafe { str::from_utf8_unchecked_mut(&mut heart) };
}