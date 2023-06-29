struct MyStruct {
    data: [i32; 5],
}

impl MyStruct {
    fn get_slice(&self) -> &[i32] {
        &self.data[..]
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr();
    let len = arr.len();
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    println!("{:?}", slice);
}