unsafe fn register_frames(&mut self) {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            // On macOS, `__register_frame` takes a pointer to a single FDE
            let start = self.frame_table.as_ptr();
            let end = start.add(self.frame_table.len());
            let mut current = start;

            // Walk all of the entries in the frame table and register them
            while current < end {
                let len = std::ptr::read::<u32>(current as *const u32) as usize;

                // Skip over the CIE
                if current != start {
                    __register_frame(current);
                    self.registrations.push(current as usize);
                }

                // Move to the next table entry (+4 because the length itself is not inclusive)
                current = current.add(len + 4);
            }
        } else {
            // On other platforms, `__register_frame` will walk the FDEs until an entry of length 0
            let ptr = self.frame_table.as_ptr();
            __register_frame(ptr);
            self.registrations.push(ptr as usize);
        }
    }
}
// https://github.com/PLSysSec/wasmtime-spectre/blob/c5f7a230ef8efe74f70cee349e0a09e899c64ccd/crates/jit/src/unwind/systemv.rs#L111