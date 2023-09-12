fn main() {
    let mut x = 42;
    let raw_ptr: *mut i32 = &mut x;
    let reference: &i32 = unsafe { &*raw_ptr }; 
    println!("Value: {}", reference);
}