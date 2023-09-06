pub fn default_config_dir() -> Option<PathBuf> {
    let dir = unsafe {
        CStr::from_ptr((*android_injected_glue::get_app().activity).externalDataPath)
    };
    // Some(PathBuf::from(dir.to_str().unwrap()))
}