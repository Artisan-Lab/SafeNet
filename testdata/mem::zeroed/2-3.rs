/*
https://github.com/ridiculousfish/fish-shell/blob/f77dc2451ef2c87003e7815d2666cff191f2093b/fish-rust/src/threads.rs#L248
*/

pub fn spawn<F: FnOnce() + Send + 'static>(callback: F) -> bool {
    // The spawned thread inherits our signal mask. Temporarily block signals, spawn the thread, and
    // then restore it. But we must not block SIGBUS, SIGFPE, SIGILL, or SIGSEGV; that's undefined
    // (#7837). Conservatively don't try to mask SIGKILL or SIGSTOP either; that's ignored on Linux
    // but maybe has an effect elsewhere.
    let saved_set = unsafe {
        let mut new_set: libc::sigset_t = std::mem::zeroed();
        let new_set = &mut new_set as *mut _;
        libc::sigfillset(new_set);
        libc::sigdelset(new_set, libc::SIGILL); // bad jump
        libc::sigdelset(new_set, libc::SIGFPE); // divide-by-zero
        libc::sigdelset(new_set, libc::SIGBUS); // unaligned memory access
        libc::sigdelset(new_set, libc::SIGSEGV); // bad memory access
        libc::sigdelset(new_set, libc::SIGSTOP); // unblockable
        libc::sigdelset(new_set, libc::SIGKILL); // unblockable

        let mut saved_set: libc::sigset_t = std::mem::zeroed();
        let result = libc::pthread_sigmask(libc::SIG_BLOCK, new_set, &mut saved_set as *mut _);
        assert_eq!(result, 0, "Failed to override thread signal mask!");
        saved_set
    };

    // Spawn a thread. If this fails, it means there's already a bunch of threads; it is very
    // unlikely that they are all on the verge of exiting, so one is likely to be ready to handle
    // extant requests. So we can ignore failure with some confidence.
    // We don't have to port the PTHREAD_CREATE_DETACHED logic. Rust threads are detached
    // automatically if the returned join handle is dropped.

    let result = match std::thread::Builder::new().spawn(move || {
        (callback)();
    }) {
        Ok(handle) => {
            let thread_id = handle.thread().id();
            FLOG!(iothread, "rust thread", thread_id, "spawned");
            // Drop the handle to detach the thread
            drop(handle);
            true
        }
        Err(e) => {
            eprintln!("rust thread spawn failure: {e}");
            false
        }
    };

    // Restore our sigmask
    unsafe {
        let result = libc::pthread_sigmask(
            libc::SIG_SETMASK,
            &saved_set as *const _,
            std::ptr::null_mut(),
        );
        assert_eq!(result, 0, "Failed to restore thread signal mask!");
    };

    result
}
