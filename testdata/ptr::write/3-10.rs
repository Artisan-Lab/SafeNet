fn alloc_slice_within_chunk(&mut self, l: Layout) -> Option<NonNull<[u8]>> {
    let size = l.size();
    let header = AllocHeader::new(size);
    let header_align = std::mem::align_of::<AllocHeader>();
    let header_size = std::mem::size_of::<AllocHeader>();

    // We must align to both the AllocHeader and provided layout, so that we can
    // safely `ptr::write` the header at the address of `pointer` below.
    let align_offset = self
        .current_next
        .align_offset(std::cmp::max(l.align(), header_align));
    let mut pointer = unsafe { self.current_next.add(align_offset) };
    let new_current = unsafe { pointer.add(header_size).add(size) };

    if new_current > self.current_end {
        return None;
    }

    debug_assert!(!new_current.is_null());
    debug_assert_eq!(pointer.align_offset(std::mem::align_of::<AllocHeader>()), 0);
    self.current_next = new_current;
    // Write header into memory
    unsafe { std::ptr::write(pointer as *mut AllocHeader, header) };
    // Move pointer by header_size
    pointer = unsafe { pointer.add(header_size) };

    let slice = unsafe { std::slice::from_raw_parts(pointer, size) };
    Some(NonNull::from(slice))
}
// https://github.com/facebook/hhvm/blob/eb80592a45e22de5590ccb534065984041e1da70/hphp/hack/src/shmrs/shardalloc.rs#L234