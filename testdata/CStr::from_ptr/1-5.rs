pub extern "C" fn cef_log(_file: *const c_char,
    _line: c_int,
    _severity: c_int,
    message: *const c_char) {
unsafe {
let slice = ffi::CStr::from_ptr(message);
println!("{}", str::from_utf8(slice.to_bytes()).unwrap())
}
}
