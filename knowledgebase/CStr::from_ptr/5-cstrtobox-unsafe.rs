use std::ffi::CStr;
use std::os::raw::c_char;

fn main() {
    let c_string_ptr: *const c_char = "Hello, Rust\0".as_ptr() as *const c_char;
    let c_str = unsafe { CStr::from_ptr(c_string_ptr) };
    let rust_string = c_str.to_str().expect("Invalid UTF-8");
    let boxed_string = unsafe { Box::from_raw(c_string_ptr as *mut c_char) };

    // println!("Boxed String: {}", boxed_string);

}
