fn new_data_got_entry(&mut self, id: DataId, val: *const u8) {
    let got_entry = self.new_got_entry(val);
    self.data_object_got_entries[id] = Some(got_entry);
}