fn main() {
    let raw_ptr = Box::into_raw(Box::new([1, 2, 3, 4, 5]));
    let boxed_data: Box<[i32]> = unsafe { Box::from_raw(raw_ptr) };
    let vec_data: Vec<i32> = boxed_data.into_vec();

    for num in &vec_data {
        println!("Value: {}", num);
    }

}

