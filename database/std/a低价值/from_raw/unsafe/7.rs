use std::alloc::{alloc, Layout};
use std::rc::Rc;

fn main() {
    let ptr = unsafe { alloc(Layout::new::<i32>()) } as *mut i32;
    unsafe { ptr.write(42) };
    let rc = unsafe { Rc::from_raw(ptr) };
    assert_eq!(*rc, 42);
}
