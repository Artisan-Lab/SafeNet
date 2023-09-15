fn main() {
    let mut uninitialized_value: i32;

    unsafe {
        std::ptr::write(&mut uninitialized_value, 100); 
    }

    println!("Uninitialized Value: {}", uninitialized_value); 
}
