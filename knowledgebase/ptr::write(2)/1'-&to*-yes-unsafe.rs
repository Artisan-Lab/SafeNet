use std::ptr;

fn main() {
    let mut value = 42;
    let ptr = &mut value as *mut i32;

    unsafe {
        ptr::write(ptr, 100); 
    }
    
    println!("Value: {}", value);
}
