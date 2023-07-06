#![allow(unused)]
fn main() {
    let x = 12;
    let y = &x as *const i32;

    unsafe {
        std::ptr::write_bytes(y as *mut i32,1,1);
    }
}

// 