use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn some_extern_function(s: *mut c_char);
}

let c_string = CString::new("Hello!").expect("CString::new failed");
let raw = c_string.as_ptr();
let len = c_string.as_bytes().len();
let new_c_string = unsafe {
    let mut vec = Vec::with_capacity(len + 1);
    vec.extend_from_slice(c_string.as_bytes());
    vec.push(0);
    CString::from_vec_unchecked(vec)
};
some_extern_function(new_c_string.as_ptr() as *mut c_char);
