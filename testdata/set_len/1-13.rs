/*
https://github.com/youknowone/coreutils/blob/44713356097bcf23c98190e6717d9e4577667567/src/uucore/src/lib/features/entries.rs#L68
*/
pub fn get_groups() -> IOResult<Vec<gid_t>> {
    let ngroups = unsafe { getgroups(0, ptr::null_mut()) };
    if ngroups == -1 {
        return Err(IOError::last_os_error());
    }
    let mut groups = Vec::with_capacity(ngroups as usize);
    let ngroups = unsafe { getgroups(ngroups, groups.as_mut_ptr()) };
    if ngroups == -1 {
        Err(IOError::last_os_error())
    } else {
        unsafe {
            groups.set_len(ngroups as usize);
        }
        Ok(groups)
    }
}