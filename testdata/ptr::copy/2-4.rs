fn compact(&mut self) {
    if self.read_pos == 0 {
        return;
    }
    let buffer = self.buffer.as_mut().unwrap();
    let ptr = buffer.as_mut_ptr();
    let readable_len = buffer.len() - self.read_pos;
    unsafe {
        std::ptr::copy(ptr.add(self.read_pos), ptr, readable_len);
        buffer.set_init(readable_len);
    }
    self.read_pos = 0;
}


// https://github.com/ihciah/shadow-tls/blob/3f4d3124b665b62d5bac44025a2781c729612a2a/src/util.rs#L582