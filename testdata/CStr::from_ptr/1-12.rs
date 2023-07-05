unsafe fn cstr_to_str<'a>(s: *const c_char) -> Option<&'a str> {
    CStr::from_ptr(s).to_str().ok()
}