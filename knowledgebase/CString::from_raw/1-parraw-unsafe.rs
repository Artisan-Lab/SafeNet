// #![allow(unused)]
// use std::ffi::CString;
// use std::os::raw::c_char;

fn free(raw: *mut c_char) {
    unsafe { CString::from_raw(raw); }
}  

// fn main() {
//     let mut cstring = CString::new("hello").expect("Failed to create CString");
//     let raw = cstring.into_raw() as *mut c_char;
//     free(raw);
// }
