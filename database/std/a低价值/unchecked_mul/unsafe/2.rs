#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroI16;

fn main() -> (){

    let two = NonZeroI16::new(2).unwrap();
    let four = NonZeroI16::new(4).unwrap();

    assert_eq!(four, unsafe { two.unchecked_mul(two) });


}
