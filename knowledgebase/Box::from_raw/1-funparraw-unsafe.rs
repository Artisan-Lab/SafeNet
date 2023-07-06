fn process_raw(raw: *mut i32) {

    let boxed_value = unsafe { Box::from_raw(raw) };
    let value = *boxed_value;
    // println!("Value: {}", value);
    
}

// fn main() {

//     let boxed_value = Box::new(42);
//     let raw_ptr = Box::into_raw(boxed_value);
    
//     process_raw(raw_ptr);

// }
