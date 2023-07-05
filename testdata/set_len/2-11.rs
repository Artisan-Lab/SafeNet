/*
https://github.com/bjorn3/rust/blob/2f896da247e0ee8f0bef7cd7c54cfbea255b9f68/library/std/src/sys/sgx/abi/usercalls/alloc.rs#L690
*/

    pub fn copy_to_enclave_vec(&self, dest: &mut Vec<T>) {
        if let Some(missing) = self.len().checked_sub(dest.capacity()) {
            dest.reserve(missing)
        }
        // SAFETY: We reserve enough space above.
        unsafe { dest.set_len(self.len()) };
        self.copy_to_enclave(&mut dest[..]);
    }