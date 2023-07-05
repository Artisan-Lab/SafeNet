unsafe extern "C" fn hackc_extract_as_json_cpp_ffi(
    flags: i32,
    filename: *const c_char,
    text_ptr: *const c_char,
    mangle_xhp: bool,
) -> *const c_char {
    match std::panic::catch_unwind(|| {
        use std::os::unix::ffi::OsStrExt;
        //  Safety : We rely on the C caller that `text_ptr` be a valid
        //  nul-terminated C string.
        let text = std::ffi::CStr::from_ptr(text_ptr).to_bytes();
        // Safety: We rely on the C caller that `filename` be a valid
        // nul-terminated C string.
        let filename = RelativePath::make(
            oxidized::relative_path::Prefix::Dummy,
            std::path::PathBuf::from(std::ffi::OsStr::from_bytes(
                std::ffi::CStr::from_ptr(filename).to_bytes(),
            )),
        );
        match extract_as_json_ffi0(
            ((1 << 0) & flags) != 0, // php5_compat_mode
            ((1 << 1) & flags) != 0, // hhvm_compat_mode
            ((1 << 2) & flags) != 0, // allow_new_attribute_syntax
            ((1 << 3) & flags) != 0, // enable_xhp_class_modifier
            ((1 << 4) & flags) != 0, // disable_xhp_element_mangling
            ((1 << 5) & flags) != 0, // disallow_hash_comments
            filename,
            text,
            mangle_xhp,
        ) {
            Some(s) => {
                let cs = std::ffi::CString::new(s)
                    .expect("rust_facts_ffi: extract_as_json_cpp_ffi: String::new failed");
                cs.into_raw() as *const c_char
            }
            None => std::ptr::null(),
        }
    }) {
        Ok(ptr) => ptr,
        Err(_) => {
            eprintln!("Error: panic in ffi function extract_as_json_cpp_ffi");
            std::ptr::null()
        }
    }
}