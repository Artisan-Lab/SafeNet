pub fn resources_dir_path() -> io::Result<PathBuf> {
    let mut dir = CMD_RESOURCE_DIR.lock().unwrap();

    if let Some(ref path) = *dir {
        return Ok(PathBuf::from(path));
    }

    let data_path = unsafe {
        CStr::from_ptr((*android_injected_glue::get_app().activity).externalDataPath)
    };
    let path = PathBuf::from(data_path.to_str().unwrap());
    *dir = Some(path.to_str().unwrap().to_owned());
    Ok(path)
}