struct MyStruct {
    data: [i32; 5],
}

impl MyStruct {
    fn get_slice(&self) -> &[i32] {
        &self.data[..]
    }
}

fn main() {
    let my_struct = MyStruct {
        data: [1, 2, 3, 4, 5],
    };
    let slice = my_struct.get_slice();
    println!("{:?}", slice);
}