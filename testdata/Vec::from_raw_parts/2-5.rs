fn drop(&mut self) {
    for (i, state) in self.states.iter().enumerate() {
        match state {
            BufState::Free { init_len, .. } => {
                // Update buffer initialization.
                // The buffer is about to be dropped, but this may release it
                // from Registry ownership, rather than deallocate.
                unsafe { self.buffers[i].set_init(*init_len) };
            }
            BufState::CheckedOut => unreachable!("all buffers must be checked in"),
        }
    }

    // Rebuild Vec<iovec>, so it's dropped
    let _ = unsafe {
        Vec::from_raw_parts(self.raw_bufs.as_ptr(), self.states.len(), self.orig_cap)
    };
}

// https://github.com/tokio-rs/tokio-uring/blob/34b27ab64e6d588aee7d43c2d7f9855b072f61ab/src/buf/fixed/plumbing/registry.rs#L137