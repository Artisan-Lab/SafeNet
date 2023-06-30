#![allow(unused)]
use std::alloc::{alloc_zeroed, dealloc, Layout};
fn main() {
    unsafe {
        let layout = Layout::new::<u16>();
        let ptr = alloc_zeroed(layout);
        dealloc(ptr, layout);
    }
}