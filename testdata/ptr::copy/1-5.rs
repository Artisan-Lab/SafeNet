pub extern "C" fn zip339_phrase_to_seed(
    language: Language,
    phrase: *const c_char,
    buf: *mut u8,
) -> bool {
    assert!(!phrase.is_null());
    assert!(!buf.is_null());

    if let Ok(language) = language.try_into() {
        if let Ok(phrase) = unsafe { CStr::from_ptr(phrase) }.to_str() {
            if let Ok(mnemonic) = zip339::Mnemonic::from_phrase_in(language, phrase) {
                // Use the empty passphrase.
                let seed = mnemonic.to_seed("");
                unsafe {
                    ptr::copy(seed.as_ptr(), buf, 64);
                }
                return true;
            }
        }
    }
    unsafe {
        ptr::write_bytes(buf, 0, 64);
    }
    false
}

// https://github.com/johnhanlon86/zcash/blob/423489c5e810d0f4deeac7bbbf8522c659ba6f8f/src/rust/src/zip339_ffi.rs#L104