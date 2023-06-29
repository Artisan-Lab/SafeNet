impl Drop for SharedBytes {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if let Raw { zero: 0, ptr } = self.0.vec {
                Arc::decrement_strong_count(ptr);
            } else {
                Arc::decrement_strong_count(self.0.slice);
            }
        }
    }
}
/*
https://github.com/TanTanDev/assets_manager/blob/dbe06334a328f3831e3accc86d2c81188bcefc79/src/utils/bytes.rs#L64
*/