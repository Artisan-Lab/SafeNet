fn main() {
    let raw_ptr = create_raw_ptr();
    let boxed_data: Box<i32> = unsafe { Box::from_raw(raw_ptr) };
    println!("Value: {}", *boxed_data);

}

fn create_raw_ptr() -> *mut i32 {
    Box::into_raw(Box::new(42))
}
