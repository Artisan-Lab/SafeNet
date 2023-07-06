#![allow(unused)]
fn main() {
    let s: &str = "hello, world!";
    let slice: &[u8];

    unsafe {
        let str_ptr: *const &str = &s;
        let slice_ptr: *const &[u8] = std::mem::transmute_copy(&str_ptr);
        slice = *slice_ptr;
    }

    println!("{:?}", slice);
}
