#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroU128;

fn main() -> (){

    let two = NonZeroU128::new(2).unwrap();
    let four = NonZeroU128::new(4).unwrap();

    assert_eq!(four, unsafe { two.unchecked_mul(two) });


}
