use std::ffi::CString;

let raw = b"foo".to_vec();
unsafe {
    let c_string = CString::from_vec_unchecked(raw);
}