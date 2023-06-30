#![allow(unused)]
fn main() {
    let s: &str = "123";

    unsafe {
        let end: *mut u8 = s.as_ptr().add(3);
        *end.sub(1);
    }
}