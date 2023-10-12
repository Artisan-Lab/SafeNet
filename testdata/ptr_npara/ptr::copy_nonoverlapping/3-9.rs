fn chunk(&self, mut file: OpenFile) -> SuccessfulRead {
    // Read a chunk that's large enough to minimize thread handoffs but
    // short enough to keep memory usage under control. It's hopefully
    // unnecessary to worry about disk seeks; the madvise call should cause
    // the kernel to read ahead.
    let end = std::cmp::min(file.map_len, file.map_pos.saturating_add(1 << 16));
    let mut chunk = Vec::new();
    let len = end.checked_sub(file.map_pos).unwrap();
    chunk.reserve_exact(len);

    // SAFETY: [map_pos, map_pos + len) is verified to be within map_ptr.
    //
    // If the read is out of bounds of the file, we'll get a SIGBUS.
    // That's not a safety violation. It also shouldn't happen because the
    // length was set properly at open time, Moonfire NVR is a closed
    // system (nothing else ever touches its files), and sample files are
    // never truncated (only appended to or unlinked).
    unsafe {
        std::ptr::copy_nonoverlapping(
            file.map_ptr.add(file.map_pos) as *const u8,
            chunk.as_mut_ptr(),
            len,
        );
        chunk.set_len(len);
    }
    let file = if end == file.map_len {
        None
    } else {
        file.map_pos = end;
        Some(file)
    };
    SuccessfulRead { chunk, file }
}
// https://github.com/scottlamb/moonfire-nvr/blob/aa60bc991c94a1e6eb5a9d7b8f670e92207d3a0f/server/db/dir/reader.rs#L413