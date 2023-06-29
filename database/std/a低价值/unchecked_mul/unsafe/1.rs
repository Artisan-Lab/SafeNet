#![allow(unused)]
#![feature(nonzero_ops)]

use std::num::NonZeroI128;

fn main() -> () {
    let two = NonZeroI128::new(2).unwrap();
    let four = NonZeroI128::new(4).unwrap();

    assert_eq!(four, unsafe { two.unchecked_mul(two) });
}
