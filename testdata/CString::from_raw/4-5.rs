pub fn build(&self) -> Result<FasterKv, FasterError<'static>> {
    if !(self.log_mutable_fraction > 0.0 && self.log_mutable_fraction <= 1.0) {
        return Err(FasterError::BuilderError(
            "Log mutable fraction must be between 0 and 1",
        ));
    }
    unsafe {
        let mut storage_dir = None;
        let faster_t = match self.storage {
            None => ffi::faster_open(self.table_size, self.log_size, self.pre_allocate_log),
            Some(path) => {
                let storage_str = CString::new(path).unwrap();
                let ptr_raw = storage_str.into_raw();
                let ft = ffi::faster_open_with_disk(
                    self.table_size,
                    self.log_size,
                    ptr_raw,
                    self.log_mutable_fraction,
                    self.pre_allocate_log,
                );
                storage_dir = CString::from_raw(ptr_raw).into_string().ok();
                ft
            }
        };
        Ok(FasterKv {
            faster_t,
            storage_dir,
        })
    }
}
// https://github.com/faster-rs/faster-rs/blob/b268fe819287d7f76e944d8df3b5b3b50ee29968/src/builder.rs#L58