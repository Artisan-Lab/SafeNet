#![allow(unused)]
fn main() {
    let x = 12;
    let y = &x as *const i32;

    unsafe {
        std::ptr::drop_in_place(y as *mut i32);
    }
}
