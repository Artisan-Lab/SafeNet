pub fn grow(&mut self, shift: u8) {
    let old_capacity = self.entries.len();
    let new_capacity = self.entries.len() << shift;
    unsafe {
        self.entries.grow_zeroed(new_capacity);
    }
    for i in 0..old_capacity {
        unsafe {
            assume(i < self.entries.len());
        }
        if K::is_zero(&self.entries[i].key) {
            continue;
        }
        let key = unsafe { self.entries[i].key.assume_init_ref() };
        let hash = K::hash(key);
        let index = (hash as usize) & (self.entries.len() - 1);
        for j in (index..self.entries.len()).chain(0..index) {
            unsafe {
                assume(j < self.entries.len());
            }
            if j == i {
                break;
            }
            if self.entries[j].is_zero() {
                unsafe {
                    self.entries[j] = std::ptr::read(&self.entries[i]);
                    self.entries[i].key = MaybeUninit::zeroed();
                }
                break;
            }
        }
    }
    for i in old_capacity..new_capacity {
        unsafe {
            assume(i < self.entries.len());
        }
        if K::is_zero(&self.entries[i].key) {
            break;
        }
        let key = unsafe { self.entries[i].key.assume_init_ref() };
        let hash = K::hash(key);
        let index = (hash as usize) & (self.entries.len() - 1);
        for j in (index..self.entries.len()).chain(0..index) {
            unsafe {
                assume(j < self.entries.len());
            }
            if j == i {
                break;
            }
            if self.entries[j].is_zero() {
                unsafe {
                    self.entries[j] = std::ptr::read(&self.entries[i]);
                    self.entries[i].key = MaybeUninit::zeroed();
                }
                break;
            }
        }
    }
}
// https://github.com/STMicroelectronics/linux/blob/d33b43a4dcc4ae3cd178793c139756af77e42bde/rust/alloc/slice.rs#L911