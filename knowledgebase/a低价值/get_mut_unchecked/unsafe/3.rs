#![feature(get_mut_unchecked)]

use std::sync::Arc;

let x: Arc<str> = Arc::from("Hello, world!");
let mut y: Arc<[u8]> = x.clone().into();
unsafe {
    // this is Undefined Behavior, because x's inner type is str, not [u8]
    Arc::get_mut_unchecked(&mut y).fill(0xff); // 0xff is invalid in UTF-8
}
println!("{}", &*x); // Invalid UTF-8 in a str