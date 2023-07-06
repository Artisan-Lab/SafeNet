fn main() {

    let raw_ptr =  Box::into_raw(Box::new(42));
    let mut boxed_data: Box<i32> = unsafe { Box::from_raw(raw_ptr) };

    *boxed_data += 10;

    // println!("Value: {}", *boxed_data);

}


