pub fn from_cstr(s: *const c_char) -> ResultType<Self> {
    let s = unsafe { CStr::from_ptr(s) };
    Ok(serde_json::from_str(s.to_str()?)?)
}