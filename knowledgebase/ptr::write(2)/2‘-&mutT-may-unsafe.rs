struct MyStruct {
    data: i32,
}

fn main() {
    let mut my_struct = MyStruct { data: 42 };
    unsafe {
        std::ptr::write(&mut my_struct.data, 123);
    }
    println!("Updated data: {}", my_struct.data);
}
