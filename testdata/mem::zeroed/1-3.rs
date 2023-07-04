/*
From: https://github.com/topjohnwu/Magisk/blob/43b9a09c9b5915924ca297e75df0773b39ad610a/native/src/core/logging.rs#L187
*/

fn zygisk_log_write(prio: i32, msg: &[u8]) {
    let magiskd = match MAGISKD.get() {
        None => return,
        Some(s) => s,
    };

    let logd_cell = magiskd.logd.lock().unwrap();
    let mut logd_ref = logd_cell.borrow_mut();
    if logd_ref.is_none() {
        android_logging();
        unsafe {
            let fd = zygisk_fetch_logd();
            if fd < 0 {
                return;
            }
            *logd_ref = Some(File::from_raw_fd(fd));
        }
        // Only re-enable zygisk logging if success
        zygisk_logging();
    };
    let logd = logd_ref.as_mut().unwrap();

    // Block SIGPIPE
    let mut mask: sigset_t;
    let mut orig_mask: sigset_t;
    unsafe {
        mask = std::mem::zeroed();
        orig_mask = std::mem::zeroed();
        sigaddset(&mut mask, SIGPIPE);
        pthread_sigmask(SIG_BLOCK, &mask, &mut orig_mask);
    }

    let result = do_magisk_log_write(logd, prio, msg);

    // Consume SIGPIPE if exists, then restore mask
    unsafe {
        let ts: timespec = std::mem::zeroed();
        sigtimedwait(&mask, null_mut(), &ts);
        pthread_sigmask(SIG_SETMASK, &orig_mask, null_mut());
    }

    // If any error occurs, shut down the logd pipe
    if result.is_err() {
        *logd_ref = None;
    }
}
