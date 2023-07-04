fn generate_string_test() {
    let generator = RandomString(4);

    let value = pactffi_generator_generate_string(&generator, std::ptr::null());
    expect!(value.is_null()).to(be_false());
    let string = unsafe { CString::from_raw(value as *mut c_char) };
    expect!(string.to_string_lossy().len()).to(be_equal_to(4));
  }