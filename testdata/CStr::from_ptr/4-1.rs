/*
    From: https://github.com/rust-lang/rust/blob/ed1ce580ec27ffcfda1a95512e69185442f75ebe/library/std/src/sys/unix/os.rs#L141
*/
pub fn error_string(errno: i32) -> String {
    extern "C" {
        #[cfg_attr(
            all(any(target_os = "linux", target_env = "newlib"), not(target_env = "ohos")),
            link_name = "__xpg_strerror_r"
        )]
        fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: libc::size_t) -> c_int;
    }

    let mut buf = [0 as c_char; TMPBUF_SZ];

    let p = buf.as_mut_ptr();
    unsafe {
        if strerror_r(errno as c_int, p, buf.len()) < 0 {
            panic!("strerror_r failure");
        }

        let p = p as *const _;
        // We can't always expect a UTF-8 environment. When we don't get that luxury,
        // it's better to give a low-quality error message than none at all.
        String::from_utf8_lossy(CStr::from_ptr(p).to_bytes()).into()
    }
}


