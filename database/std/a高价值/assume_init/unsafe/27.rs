#![feature(new_uninit)]
#![feature(get_mut_unchecked)]

use std::sync::Arc;

let mut five = Arc::<u32>::new_uninit();

// Deferred initialization:
Arc::get_mut(&mut five).unwrap().write(5);

let five = unsafe { five.assume_init() };

assert_eq!(*five, 5)