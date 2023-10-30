pub fn args() -> Args {
    use crate::os::unix::prelude::*;
    extern "C" {
        // These functions are in crt_externs.h.
        fn _NSGetArgc() -> *mut libc::c_int;
        fn _NSGetArgv() -> *mut *mut *mut libc::c_char;
    }

    let vec = unsafe {
        let (argc, argv) =
            (*_NSGetArgc() as isize, *_NSGetArgv() as *const *const libc::c_char);
        (0..argc as isize)
            .map(|i| {
                let bytes = CStr::from_ptr(*argv.offset(i)).to_bytes().to_vec();
                OsStringExt::from_vec(bytes)
            })
            .collect::<Vec<_>>()
    };
    Args { iter: vec.into_iter() }
}