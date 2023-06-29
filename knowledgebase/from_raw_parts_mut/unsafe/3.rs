struct MyStruct {
    data: [i32; 5],
}

impl MyStruct {
    fn get_slice(&mut self) -> &mut [i32] {
        &mut self.data[..]
    }
}

fn main() {
    let mut my_struct = MyStruct { data: [0; 5] };
    let ptr = my_struct.get_slice().as_mut_ptr();
    let len = my_struct.get_slice().len();
    let slice = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    slice.copy_from_slice(&[1, 2, 3, 4, 5]);
    println!("{:?}", my_struct.data);
}
