#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroU64;

fn main() -> (){

    let two = NonZeroU64::new(2).unwrap();
    let four = NonZeroU64::new(4).unwrap();

    assert_eq!(four, unsafe { two.unchecked_mul(two) });


}
