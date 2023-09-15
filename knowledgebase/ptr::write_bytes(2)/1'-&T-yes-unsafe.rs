use std::ptr;

fn main() {
    let x: i32 = 42;
    let raw_ptr: *const i32 = &x;
    let read_value: i32;

    unsafe {
        read_value = ptr::read(raw_ptr); 
    }

    println!("Read value: {}", read_value);
}
