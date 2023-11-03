fn sp_index(&self, sp: StackPtr) -> isize {
    unsafe {
        let ptr = self.stack.as_mut_ptr();
        sp.as_ptr().offset_from(ptr)
    }
}
//https://github.com/sisshiki1969/ruruby/blob/1c9b7d7738331d2fb9bbe6ae37e261b34317cebb/ruruby/src/vm/executor/frame.rs#L409