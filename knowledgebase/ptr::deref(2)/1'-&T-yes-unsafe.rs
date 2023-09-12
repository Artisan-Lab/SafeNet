fn main() {
    let x = 42;
    let raw_ptr: *const i32 = &x;
    let value: i32 = unsafe { *raw_ptr }; 
    println!("Value: {}", value);
}
