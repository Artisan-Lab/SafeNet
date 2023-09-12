fn main() {
    let mut x = 42;
    let raw_ptr: *mut i32 = &mut x;
    let mut_ref: &mut i32 = unsafe { &mut *raw_ptr };
    *mut_ref += 1; // UB
    println!("Value: {}", x);
}

