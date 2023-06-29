struct MyStruct {
    data: [i32; 5],
}

impl MyStruct {
    fn get_slice_mut(&mut self) -> &mut [i32] {
        &mut self.data[..]
    }
}

fn main() {
    let mut my_struct = MyStruct {
        data: [1, 2, 3, 4, 5],
    };
    let slice = my_struct.get_slice_mut();
    slice[0] = 10;
    println!("{:?}", slice);
}