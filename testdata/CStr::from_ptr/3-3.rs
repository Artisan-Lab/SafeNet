fn init_locale(vars: &EnvStack) {
    // #[rustfmt::skip]
    // const UTF8_LOCALES: &[&str] = &[
    //     "C.UTF-8", "en_US.UTF-8", "en_GB.UTF-8", "de_DE.UTF-8", "C.utf8", "UTF-8",
    // ];

    let old_msg_locale: CString = unsafe {
        let old = libc::setlocale(libc::LC_MESSAGES, std::ptr::null());
        // We have to make a copy because the subsequent setlocale() call to change the locale will
        // invalidate the pointer from this setlocale() call.
        CStr::from_ptr(old.cast()).to_owned()
    };

    // for var_name in LOCALE_VARIABLES {
    //     let var = vars
    //         .getf_unless_empty(var_name, EnvMode::EXPORT)
    //         .map(|v| v.as_string());
    //     if let Some(value) = var {
    //         FLOGF!(env_locale, "locale var", var_name, "=", value);
    //         setenv_lock(var_name, &value, true);
    //     } else {
    //         FLOGF!(env_locale, "locale var", var_name, "is missing or empty");
    //         unsetenv_lock(var_name);
    //     }
    }