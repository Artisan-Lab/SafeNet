fn mem_end(&self) -> *const u8 {
    unsafe { self.memory.as_ptr().add(self.memory.len()) }
    
}