pub fn run_as_user(uname: &str) {
    use log::warn;
    use std::{
        ffi::{CStr, CString},
        io::Error,
    };

    unsafe {
        let pwd = match uname.parse::<libc::uid_t>() {
            Ok(uid) => {
                let mut pwd = libc::getpwuid(uid);
                if pwd.is_null() {
                    let uname = CString::new(uname).expect("username");
                    pwd = libc::getpwnam(uname.as_ptr())
                }
                pwd
            }
            Err(..) => {
                let uname = CString::new(uname).expect("username");
                libc::getpwnam(uname.as_ptr())
            }
        };

        if pwd.is_null() {
            warn!("user {} not found", uname);
            return;
        }

        let pwd = &*pwd;

        // setgid first, because we may not allowed to do it anymore after setuid
        if libc::setgid(pwd.pw_gid as libc::gid_t) != 0 {
            let err = Error::last_os_error();

            warn!(
                "could not change group id to user {:?}'s gid: {}, uid: {}, error: {}",
                CStr::from_ptr(pwd.pw_name),
                pwd.pw_gid,
                pwd.pw_uid,
                err
            );
            return;
        }

        if libc::initgroups(pwd.pw_name, pwd.pw_gid.try_into().unwrap()) != 0 {
            let err = Error::last_os_error();
            warn!(
                "could not change supplementary groups to user {:?}'s gid: {}, uid: {}, error: {}",
                CStr::from_ptr(pwd.pw_name),
                pwd.pw_gid,
                pwd.pw_uid,
                err
            );
            return;
        }

        if libc::setuid(pwd.pw_uid) != 0 {
            let err = Error::last_os_error();
            warn!(
                "could not change user id to user {:?}'s gid: {}, uid: {}, error: {}",
                CStr::from_ptr(pwd.pw_name),
                pwd.pw_gid,
                pwd.pw_uid,
                err
            );
            return;
        }
    }
}