fn from(s: PathBuf) -> Rc<Path> {
    let rc: Rc<OsStr> = Rc::from(s.into_os_string());
    unsafe { Rc::from_raw(Rc::into_raw(rc) as *const Path) }
}