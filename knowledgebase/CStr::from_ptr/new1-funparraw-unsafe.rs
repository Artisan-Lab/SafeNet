use std::ffi::CStr;
fn main() {

    let c_string = std::ffi::CString::new("Hello, C string!").unwrap();
    let raw_ptr = c_string.as_ptr();
    let cstr = unsafe { CStr::from_ptr(raw_ptr) };
    
    if let Ok(string) = cstr.to_str() {
        println!("String: {}", string);
    } else {
        println!("Invalid C string");
    }

}
