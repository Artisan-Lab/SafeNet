// use std::ffi::CStr;
// use std::os::raw::c_char;

fn modify_c_string(c_str_ptr: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(c_str_ptr) };
    let mut modified_str = c_str.to_bytes().to_vec();

    modified_str[0] = b'H';
    modified_str[1] = b'e';
    modified_str[2] = b'l';
    modified_str[3] = b'l';
    modified_str[4] = b'o';
    let modified_c_str = CStr::from_bytes_with_nul(&modified_str)
        .expect("Failed to create modified CStr");
    println!("Modified string: {:?}", modified_c_str);
}

fn main() {
    let c_str_ptr: *const c_char = b"world\0".as_ptr() as *const c_char;
    modify_c_string(c_str_ptr);
}
