pub unsafe fn readlink_unsafe(path: *const c_char, buf: *mut u8, bufsz: usize) -> isize {
    let r = libc::readlink(path, buf.cast(), bufsz - 1);
    if r >= 0 {
        *buf.offset(r) = b'\0';
    }
    r
}
// https://github.com/topjohnwu/Magisk/blob/606d97ae4d60b4e903ef2944bd3f3fc409a9a2db/native/src/base/files.rs#L36