use std::ffi::CString;
use std::os::raw::c_char;

// struct MyStruct {
//     cstring: *mut c_char,
// }

// impl MyStruct {
//     fn new(cstring: *mut c_char) -> Self {
//         Self { cstring }
//     }

    fn process_cstring(self) {
        unsafe {
            let cstring = CString::from_raw(self.cstring);
            let rust_string = cstring.into_string().expect("Failed to convert CString to String");

            println!("Rust String: {}", rust_string);
        }
    }
// }

// fn main() {
//     let cstr_value = CString::new("Hello, World!").expect("Failed to create CString").into_raw();
//     let my_struct = MyStruct::new(cstr_value);
//     my_struct.process_cstring();

// }
