#![allow(unused)]
fn main() {
    let s: &str = "123";
    let ptr: *const u8 = s.as_ptr();

    unsafe {
        let x = *ptr.offset(1) as char;
    }
}
