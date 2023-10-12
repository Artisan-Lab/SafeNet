// impl SlabEntry {
    fn self_ref(&self) -> &HttpSlabRecord {
        // SAFETY: We have the lock and we're borrowing lifetime from self
        unsafe { self.0.as_ref() }
    }
// }
// https://github.com/denoland/deno/blob/cd2525d4cff4e18b1e3e7d29458079942ba2b6c5/ext/http/slab.rs#L143