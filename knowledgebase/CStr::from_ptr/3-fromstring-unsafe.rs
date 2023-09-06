// use std::ffi::CStr;
// use std::os::raw::c_char;

// extern "C" {
//     fn get_string() -> *const c_char;
// }

fn main() {
    unsafe {
        let c_string_ptr = get_string();
        let c_str = CStr::from_ptr(c_string_ptr);
    }
}
