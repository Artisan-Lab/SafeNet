fn main() {
    
    let mut data: [u8; 4] = [0; 4];
    let uninitialized_ptr: *mut u8 = std::ptr::null_mut();

    unsafe {
        std::ptr::write_bytes(uninitialized_ptr, 0xAA, 4);
    }

    println!("{:?}", data);
}
