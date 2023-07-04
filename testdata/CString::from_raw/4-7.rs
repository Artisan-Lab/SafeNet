pub fn load_config(&self, path: &str) -> Result<()> {
    let file = CString::new(path)?.into_raw();
    let ret = mpv_err((), unsafe {
        libmpv_sys::mpv_load_config_file(self.ctx.as_ptr(), file)
    });
    unsafe { CString::from_raw(file) };
    ret
}