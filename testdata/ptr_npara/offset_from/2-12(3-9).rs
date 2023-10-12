pub fn gc_check_and_mark(&mut self, ptr: &T) -> bool {
    let ptr = ptr as *const T as *mut T;
    #[cfg(feature = "gc-debug")]
    self.check_ptr(ptr);
    let mut page_ptr = PageRef::from_inner(ptr);

    let index = unsafe { ptr.offset_from(page_ptr.get_data_ptr(0)) } as usize;
    assert!(index < DATA_LEN);
    let bit_mask = 1 << (index % 64);
    let bitmap = &mut page_ptr.mark_bits_mut()[index / 64];

    let is_marked = (*bitmap & bit_mask) != 0;
    *bitmap |= bit_mask;
    if !is_marked {
        self.mark_counter += 1;
    }
    is_marked
}
//https://github.com/sisshiki1969/ruruby/blob/1c9b7d7738331d2fb9bbe6ae37e261b34317cebb/ruruby/src/alloc.rs#L258