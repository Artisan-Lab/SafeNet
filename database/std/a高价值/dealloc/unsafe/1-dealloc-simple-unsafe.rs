#![allow(unused)]
use std::alloc::{alloc, dealloc, Layout};
fn main() {
    unsafe {
        let layout = Layout::new::<u16>();
        let p = alloc(layout);
        dealloc(p, layout);
    }
}