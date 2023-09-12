fn main() {
    let mut x = 42;
    let raw_ptr: *mut i32 = &mut x;
    let reference: &mut i32 = &mut x; // another mut ref
    let value: i32 = unsafe { *raw_ptr }; 
    println!("Value: {}", value);
}
