// use std::ffi::CString;
// use std::os::raw::c_char;

fn main() {
    // Create a CString
    let mut cstring = CString::new("hello").expect("Failed to create CString");
    let cstring_ptr = cstring.as_ptr() as *mut c_char;
    let mut index = 0;
    let mut c = unsafe { *cstring_ptr.offset(index) };
    while c != 0 {
        unsafe {
            *cstring_ptr.offset(index) = c + 1;
        }
        index += 1;
        c = unsafe { *cstring_ptr.offset(index) };
    }
    let modified_cstring = unsafe { CString::from_raw(cstring_ptr) };
    println!("Modified CString (while loop): {:?}", modified_cstring);
}
