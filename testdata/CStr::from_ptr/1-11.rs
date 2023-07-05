pub extern "C" fn ts_highlighter_add_language(
    this: *mut TSHighlighter,
    scope_name: *const c_char,
    language: Language,
    property_sheet_json: *const c_char,
    injection_regex: *const c_char,
) -> ErrorCode {
    let this = unwrap_mut_ptr(this);
    let scope_name = unsafe { CStr::from_ptr(scope_name) };
    let scope_name = unwrap(scope_name.to_str()).to_string();
    let property_sheet_json = unsafe { CStr::from_ptr(property_sheet_json) };
    let property_sheet_json = unwrap(property_sheet_json.to_str());

    let property_sheet = unwrap(load_property_sheet(language, property_sheet_json));
    let injection_regex = if injection_regex.is_null() {
        None
    } else {
        let pattern = unsafe { CStr::from_ptr(injection_regex) };
        Some(unwrap(Regex::new(unwrap(pattern.to_str()))))
    };

    this.languages.insert(
        scope_name,
        LanguageConfiguration {
            language,
            property_sheet,
            injection_regex,
        },
    );

    ErrorCode::Ok
}