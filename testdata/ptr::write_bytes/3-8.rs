fn init(&mut self) {
    self.locals_uninit().fill(MaybeUninit::new(None));

    if cfg!(debug_assertions) {
        // Write junk to the stack to trigger memory error if the stack is used incorrectly.
        unsafe {
            // Any bit pattern would do except
            // * a bit pattern which would make valid int tag (0b_xxxx_x010)
            // * zeros, which would be interpreted as uninitialized if copied to a local
            let byte = 0xef;

            let start = self.stack_uninit().as_ptr() as *mut u8;
            ptr::write_bytes(
                start,
                byte,
                (self.max_stack_size as usize) * mem::size_of::<Value>(),
            );
        }
    }
}
// https://github.com/facebook/buck2/blob/1835e7fd21530ff58ad9f690e8723d4e13131462/starlark-rust/starlark/src/eval/bc/frame.rs#L225