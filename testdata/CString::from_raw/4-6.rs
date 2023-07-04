pub fn prop_str(&self, prop: sys::zpool_prop_t::Type) -> Result<CString> {
    let s = String::with_capacity(sys::ZPOOL_MAXPROPLEN as usize);
    let c_string = CString::new(s).unwrap();
    let raw = c_string.into_raw();

    unsafe {
        let r = sys::zpool_get_prop(
            self.raw,
            prop,
            raw,
            sys::ZPOOL_MAXPROPLEN as usize,
            ptr::null_mut(),
            sys::boolean::B_FALSE,
        );

        let out = CString::from_raw(raw);

        if r != 0 {
            Err(::std::io::Error::from_raw_os_error(r))?
        } else {
            Ok(out)
        }
    }
}

// https://github.com/whamcloud/rust-libzfs/blob/6041b5e746cdaff54552b9c53163686378f74bdf/libzfs/src/zpool.rs#L62