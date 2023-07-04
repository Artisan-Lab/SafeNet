/*
https://github.com/facebook/hhvm/blob/eb80592a45e22de5590ccb534065984041e1da70/hphp/hack/src/shmrs/sync.rs#L110
*/

pub unsafe fn initialize(&self) -> Result<RwLockRef<'_, T>, LockError> {
        let mut attr: libc::pthread_rwlockattr_t = std::mem::zeroed();

        Errno::from(libc::pthread_rwlockattr_init(&mut attr as *mut _))?;

        Self::set_prefer_writer(&mut attr as *mut _)?;

        // Allow access from multiple processes
        Errno::from(libc::pthread_rwlockattr_setpshared(
            &mut attr as *mut _,
            libc::PTHREAD_PROCESS_SHARED,
        ))?;
        Errno::from(libc::pthread_rwlock_init(
            self.lock_ptr(),
            &attr as *const _,
        ))?;

        Ok(self.attach())
    }