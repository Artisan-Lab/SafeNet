use std::ptr;

fn main() {
    let mut x: i32 = 42;
    let raw_ptr: *mut i32 = &mut x;
    let read_value: i32;

    unsafe {
        read_value = ptr::read(raw_ptr); 
    }

    println!("Read value: {}", read_value);
}
