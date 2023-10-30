fn is_mounted(mp: impl AsRef<Path>) -> Result<bool> {
    let mp = mp
        .as_ref()
        .to_str()
        .ok_or_else(|| Error::from_raw_os_error(libc::EINVAL))?;
    let mp = CString::new(String::from(mp)).map_err(|_| Error::from_raw_os_error(libc::EINVAL))?;
    let mut mpb: Vec<libc::statfs> = Vec::new();
    let mut mpb_ptr = mpb.as_mut_ptr();
    let mpb_ptr = &mut mpb_ptr;

    let mpb: Vec<libc::statfs> = unsafe {
        let res = libc::getmntinfo(mpb_ptr, libc::MNT_NOWAIT);
        if res < 0 {
            return Err(Error::from_raw_os_error(res));
        }
        let size = res as usize;
        Vec::from_raw_parts(*mpb_ptr, size, size)
    };
    let match_mp = mpb.iter().find(|mp_stat| unsafe {
        let mp_name = CStr::from_ptr(&mp_stat.f_mntonname as *const i8);
        let mp = CStr::from_ptr(mp.as_ptr());
        mp.eq(mp_name)
    });

    Ok(match_mp.is_some())
}