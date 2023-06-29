#![feature(get_mut_unchecked)]

use std::sync::Arc;

let mut x = Arc::new(String::new());
unsafe {
    Arc::get_mut_unchecked(&mut x).push_str("foo")
}
assert_eq!(*x, "foo");