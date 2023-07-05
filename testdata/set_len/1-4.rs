/*
https://github.com/zed-industries/wezterm/blob/9ed5c084b63ea597147c2a95bb69889b9a9aa6c7/procinfo/src/macos.rs#L64
*/

 pub fn executable_path(pid: u32) -> Option<PathBuf> {
        let mut buffer: Vec<u8> = Vec::with_capacity(libc::PROC_PIDPATHINFO_MAXSIZE as _);
        let x = unsafe {
            libc::proc_pidpath(
                pid as _,
                buffer.as_mut_ptr() as *mut _,
                libc::PROC_PIDPATHINFO_MAXSIZE as _,
            )
        };
        if x <= 0 {
            return None;
        }

        unsafe { buffer.set_len(x as usize) };
        Some(OsString::from_vec(buffer).into())
    }