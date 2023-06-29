#![allow(unused)]
fn main() {
    use std::sync::Arc;

    let five = Arc::new(5);

    unsafe {
        let ptr = Arc::into_raw(five);
        Arc::increment_strong_count(ptr);
    }
}