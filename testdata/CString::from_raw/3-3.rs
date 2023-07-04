fn chown(path: &str, uid: u32, gid: u32) -> Result<c_int> {
    let r_path = match validate_raw_path(path) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    unsafe {
        let res = libc::chown(r_path, uid, gid);
        let _ = CString::from_raw(r_path); // necessary to prevent leaks
        Ok(res)
    }
}

// https://github.com/habitat-sh/habitat/blob/5986fee1f106267aca12f97e92c0443bdc4436da/components/core/src/util/posix_perm.rs#L104