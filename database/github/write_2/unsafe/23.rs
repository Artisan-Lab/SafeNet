fn new_data_got_entry(&mut self, id: DataId, val: *const u8) {
    let got_entry = self
        .memory
        .writable
        .allocate(
            std::mem::size_of::<*const u8>(),
            std::mem::align_of::<*const u8>().try_into().unwrap(),
        )
        .unwrap()
        .cast::<*const u8>();
    self.data_object_got_entries[id] = Some(NonNull::new(got_entry).unwrap());
    unsafe {
        std::ptr::write(got_entry, val);
    }
}