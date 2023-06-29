#![feature(get_mut_unchecked)]

use std::sync::Arc;

let x: Arc<&str> = Arc::new("Hello, world!");
{
    let s = String::from("Oh, no!");
    let mut y: Arc<&str> = x.clone().into();
    unsafe {
        // this is Undefined Behavior, because x's inner type
        // is &'long str, not &'short str
        *Arc::get_mut_unchecked(&mut y) = &s;
    }
}
println!("{}", &*x); // Use-after-free