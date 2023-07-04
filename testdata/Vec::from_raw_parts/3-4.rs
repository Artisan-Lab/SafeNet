fn drop(&mut self) {
    let ptr = self.ptr.get() as *mut u64;
    let cap = self.cap.get() / 8;
    unsafe {
        Vec::from_raw_parts(ptr, 0, cap);
    }
}

// https://github.com/tikv/agatedb/blob/bb041a778190a7fe9af60fc35ae78320f1c01f1e/skiplist/src/arena.rs#L21