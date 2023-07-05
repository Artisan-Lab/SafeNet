pub extern "C" fn zip339_validate_phrase(language: Language, phrase: *const c_char) -> bool {
    assert!(!phrase.is_null());

    if let Ok(language) = language.try_into() {
        if let Ok(phrase) = unsafe { CStr::from_ptr(phrase) }.to_str() {
            return zip339::Mnemonic::validate_in(language, phrase).is_ok();
        }
    }
    false
}