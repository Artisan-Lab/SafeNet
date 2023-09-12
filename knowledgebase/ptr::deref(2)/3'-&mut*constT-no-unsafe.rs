fn main() {
    let x = 42;
    let raw_ptr: *const i32 = &x;
    let mut_ref: &mut i32 = unsafe { &mut *(raw_ptr as *mut i32) }; 
    *mut_ref += 1; //data
    println!("Value: {}", x);
}

