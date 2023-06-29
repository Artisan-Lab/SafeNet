unsafe extern "C" fn translate(name: *const c_char, locale: *const c_char) -> *const c_char {
    let name = CStr::from_ptr(name);
    let locale = CStr::from_ptr(locale);
    let res = if let (Ok(name), Ok(locale)) = (name.to_str(), locale.to_str()) {
        crate::client::translate_locale(name.to_owned(), locale)
    } else {
        String::new()
    };
    CString::from_vec_unchecked(res.into_bytes()).into_raw()
}
/*
https://github.com/rustdesk/rustdesk/blob/50c1015e8651c622f7539956d24ca427b4a45cf8/src/flutter_ffi.rs#L1279
*/