// struct MyStruct {
//     data: *mut i32,
//     len: usize,
//     capacity: usize,
// }

// impl MyStruct {
//     fn new(data: *mut i32, len: usize, capacity: usize) -> Self {
//         Self {
//             data,
//             len,
//             capacity,
//         }
//     }

    fn drop(&mut self) {
        unsafe {
            let elements = Vec::from_raw_parts(self.data, self.len, self.capacity);
            drop(elements); 
        }
    }
// }

// fn main() {
//     let mut my_struct = MyStruct::new(vec![1, 2, 3].as_mut_ptr(), 3, 3);

//     my_struct.drop();
// }
