pub fn pop(&mut self) -> Option<T> {
    if self.is_empty() {
        return None;
    }
    unsafe {
        let new_len = self.len() - 1;
        self.set_len(new_len);
        Some(ptr::read(self.get_unchecked_ptr(new_len)))
    }
}