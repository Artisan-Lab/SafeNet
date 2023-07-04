use std::ffi::CString;
use std::os::raw::c_char;

// struct MyStruct {
//     cstring: CString,
// }

// impl MyStruct {
    fn new_from_raw(raw_ptr: *const c_char) -> Self {
        // 使用CString::from_raw将原始指针转换为CString对象
        let cstring = unsafe { CString::from_raw(raw_ptr as *mut c_char) };

        Self { cstring }
    }
// }

// fn main() {
//     let raw_ptr = "Hello, World!\0".as_ptr() as *const c_char;

//     let my_struct = MyStruct::new_from_raw(raw_ptr);

// }
