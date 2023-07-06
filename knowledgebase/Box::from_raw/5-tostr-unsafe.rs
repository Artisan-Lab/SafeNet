fn main() {

    let raw_ptr = Box::into_raw(vec![b'h', b'e', b'l', b'l', b'o'].into_boxed_slice());
    let boxed_data: Box<[u8]> = unsafe { Box::from_raw(raw_ptr) };
    let string_data = String::from_utf8_lossy(&boxed_data).to_string();

    // println!("Value: {}", string_data);

}

