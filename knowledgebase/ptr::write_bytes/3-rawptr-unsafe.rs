#![allow(unused)]

fn return_raw(x:i32) -> *const i32
{
    &x as *const i32
}

fn main() {
    let x = 12;

    unsafe {
        std::ptr::write_bytes(return_raw(x) as *mut u32, 1 , 1);
    }
}
