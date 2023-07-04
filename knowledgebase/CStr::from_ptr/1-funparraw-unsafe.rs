use std::ffi::CStr;
use std::os::raw::c_char;

fn modify_cstring(raw_ptr: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(raw_ptr) };
    let mut bytes = c_str.to_bytes().to_vec();
    bytes.iter_mut().for_each(|b| *b = b.to_ascii_uppercase());
    let modified_c_str = CStr::from_bytes_with_nul(&bytes).expect("Invalid C string");
    println!("Modified C string: {:?}", modified_c_str);
}

fn main() {
    let original_c_string = "hello\0".as_ptr() as *const c_char;
    modify_cstring(original_c_string);
}
