/*
https://github.com/LSPosed/Metagisk/blob/5e2ef1b7f41aa695d425db152add83629cc93363/native/src/base/files.rs#L92
*/

pub fn realpath(path: &CStr, buf: &mut [u8]) -> isize {
    if let Some(fd) = open_fd!(path, O_PATH | O_CLOEXEC) {
        let mut st1: libc::stat;
        let mut st2: libc::stat;
        let mut skip_check = false;
        unsafe {
            st1 = mem::zeroed();
            if libc::fstat(fd.as_raw_fd(), &mut st1) < 0 {
                // This shall only fail on Linux < 3.6
                skip_check = true;
            }
        }
        let len = fd_path(fd.as_raw_fd(), buf);
        unsafe {
            st2 = mem::zeroed();
            if libc::stat(buf.as_ptr().cast(), &mut st2) < 0
                || (!skip_check && (st2.st_dev != st1.st_dev || st2.st_ino != st1.st_ino))
            {
                *errno() = ENOENT;
                return -1;
            }
        }
        len
    } else {
        *errno() = ENOENT;
        -1
    }
}