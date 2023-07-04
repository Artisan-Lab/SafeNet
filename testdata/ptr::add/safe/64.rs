fn mem_end(&self) -> *const u8 {
    
    self.memory_start.wrapping_add(self.memory_len)
}