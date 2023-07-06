use std::ffi::CStr;
use std::os::raw::c_char;

// struct MyStruct {
//     value: String,
// }

// impl MyStruct {
    fn new_from_cstr(cstr: *const c_char) -> Option<Self> {
        let cstr_value = unsafe { CStr::from_ptr(cstr) };
        match cstr_value.to_str() {
            Ok(value) => Some(Self { value: value.to_owned() }),
            Err(_) => None,
        }
    }
// }

// fn main() {
//     let cstr_value = "Hello, World!\0".as_ptr() as *const c_char;
//     if let Some(my_struct) = MyStruct::new_from_cstr(cstr_value) {
//         println!("Successed to construct MyStruct");
//     } else {
//         println!("Failed to construct MyStruct");
//     }
// }
