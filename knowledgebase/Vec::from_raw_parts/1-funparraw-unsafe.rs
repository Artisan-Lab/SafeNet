fn process_raw(raw: *mut i32, len: usize, capacity: usize) {
    let vec = unsafe { Vec::from_raw_parts(raw, len, capacity) };
    for element in vec {
        println!("Element: {}", element);
    }
}

// fn main() {
   
//     let raw_ptr = Box::into_raw(vec![1, 2, 3, 4, 5].into_boxed_slice()) as *mut i32;
//     let len = 5;
//     let capacity = 5;
    
//     process_raw(raw_ptr, len, capacity);

// }
