pub fn take_fsm(&self) -> Option<Box<N>> {
    let res = self.status.compare_exchange(
        Self::NOTIFYSTATE_IDLE,
        Self::NOTIFYSTATE_NOTIFIED,
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    if res.is_err() {
        return None;
    }

    let p = self.data.swap(ptr::null_mut(), Ordering::AcqRel);
    if !p.is_null() {
        Some(unsafe { Box::from_raw(p) })
    } else {
        panic!("inconsistent status and data, something should be wrong.");
    }
}

// https://github.com/sticnarf/tikv/blob/d083fc92d5228fb4bfc74a98e5ae7982d8fef22a/components/batch-system/src/fsm.rs#L110