fn main() {
    let x = 42;
    let raw_ptr: *const i32 = &x;
    let mut reference: &mut i32 = unsafe { &mut *(raw_ptr as *mut i32) }; 
    *reference = 10;
    println!("Value: {}", x);
}