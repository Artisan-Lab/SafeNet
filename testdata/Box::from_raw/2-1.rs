fn drop(&mut self) {
    unsafe {
        if self.0.as_ref().fetch_sub(1, Ordering::AcqRel) == 1 {
            drop(Box::from_raw(self.0.as_ptr()));
        }
    }
}

// https://github.com/gfx-rs/wgpu/blob/17143c149c13ea5af36909ef6033e776cecad28c/wgpu-core/src/lib.rs#L143