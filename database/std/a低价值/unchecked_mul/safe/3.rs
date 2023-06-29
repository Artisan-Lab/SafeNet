#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroI32;

fn main() -> (){

    let two = NonZeroI32::new(2).unwrap();
    let four = NonZeroI32::new(4).unwrap();

    assert_eq!(Some(four), two.checked_mul(two) );


}
