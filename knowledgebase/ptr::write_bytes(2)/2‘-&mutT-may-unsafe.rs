fn main() {
    let mut arr = [1, 2, 3, 4, 5];

    unsafe {
        std::ptr::write_bytes(arr.as_mut_ptr(), 0, arr.len());
    }

    println!("Array after zeroing: {:?}", arr);
}

