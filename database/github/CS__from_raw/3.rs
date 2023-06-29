fn test_rust_make_temp() {
    let mut tmpdir = env::temp_dir();
    tmpdir.push(".emacs-XXXXXX");
    let fullpath = tmpdir.to_string_lossy().into_owned();
    let name = CString::new(fullpath).unwrap();
    let name_copy = name.clone();
    let raw_ptr = name.into_raw();
    let save_errno = errno::errno();
    let file_handle = unsafe { rust_make_temp(raw_ptr, 0) };
    assert!(file_handle != -1);
    let new_name = unsafe { CString::from_raw(raw_ptr) };
    assert!(new_name != name_copy);
    assert!(save_errno == errno::errno());
}
/*
https://github.com/harryfei/remacs/blob/5b59c0f77113a1330853cff077a08f00c8f7fc88/rust_src/remacs-lib/files.rs#L165
*/