#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroI64;

fn main() -> (){

    let two = NonZeroI64::new(2).unwrap();
    let four = NonZeroI64::new(4).unwrap();

    assert_eq!(four, unsafe { two.unchecked_mul(two) });


}
