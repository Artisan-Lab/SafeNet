#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroU128;

fn main() -> (){

    let one = NonZeroU128::new(1).unwrap();
    let two = NonZeroU128::new(2).unwrap();

    assert_eq!(two, unsafe { one.unchecked_add(1) });


}
