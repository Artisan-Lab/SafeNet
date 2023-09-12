use std::ptr;

fn main() {
    let x: i32 = 42;
    let raw_ptr: *const i32 = &x;
    let read_value: i32;

    unsafe {
        let mut writable_ptr: *mut i32 = raw_ptr as *mut i32;
        read_value = ptr::read(writable_ptr); 
    }

    println!("Read value: {}", read_value);
}
