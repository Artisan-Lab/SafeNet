unsafe fn deallocate_segment(&mut self, ptr: *mut u8, word_size: u32, words_used: u32) {
    if ptr == self.scratch_space.as_mut_ptr() {
        // Rezero the slice to allow reuse of the allocator. We only need to write
        // words that we know might contain nonzero values.
        unsafe {
            core::ptr::write_bytes(ptr, 0u8, (words_used as usize) * BYTES_PER_WORD);
        }
        self.scratch_space_allocated = false;
    } else {
        self.allocator
            .deallocate_segment(ptr, word_size, words_used);
    }
}
// https://github.com/capnproto/capnproto-rust/blob/dd9c072ce48c8e5377b3844ecaca832749eece08/capnp/src/message.rs#L856