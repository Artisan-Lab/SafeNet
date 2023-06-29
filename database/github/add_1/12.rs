fn entry(&self, idx: usize) -> MemoryEntry {
    assert!(idx < self.num_entries());
    let ptr = self.memmap_paddr as *const MemMapEntry;
    let entry = unsafe { *ptr.add(idx) };
    MemoryEntry::from(entry)
}