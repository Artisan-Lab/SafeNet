fn push_head_ptr(&mut self, ptr: *mut Node) {
    if !self.head.is_null() {
        unsafe {
            (*self.head).next = ptr;
            (*ptr).prev = self.head;
        }
    }

    if self.tail.is_null() {
        self.tail = ptr;
    }

    self.head = ptr;
}
// https://github.com/TilCreator/sled/blob/a0d51f20e965f505ee8b41fbc313f023f76de0bf/src/dll.rs#L119