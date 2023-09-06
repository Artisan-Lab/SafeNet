
// struct MyStruct {
//     c_string: *const i8,
// }

// impl MyStruct {
//     fn new(c_string: *const i8) -> MyStruct {
//         MyStruct { c_string }
//     }

    fn process_string(&self) {
        let c_str = unsafe { CStr::from_ptr(self.c_string) };
        let rust_str = c_str.to_str().expect("Invalid UTF-8");

        println!("String: {}", rust_str);
    }
// }

// fn main() {
//     let c_string = "Hello, World!\0".as_ptr() as *const i8;

//     let my_struct = MyStruct::new(c_string);
//     my_struct.process_string();
// }
