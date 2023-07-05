pub unsafe extern "C" fn l10nfilesource_release(source: *const FileSource) {
    let _ = Rc::from_raw(source);
}