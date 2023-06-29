#![allow(unused)]
use std::alloc::{alloc_zeroed, Layout};
fn main() {
    unsafe {
        let layout = Layout::new::<u16>();
        let ptr = alloc_zeroed(layout);
    }
}