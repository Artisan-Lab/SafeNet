/*
https://github.com/kotborealis/wasmer/blob/f1cbdf34e2925ebd09004a47599de79f6a9f9f87/lib/api/src/js/mem_access.rs#L337
*/
pub fn read_to_vec(self) -> Result<Vec<T>, MemoryAccessError> {
        let len = self.len.try_into().expect("WasmSlice length overflow");
        let mut vec = Vec::with_capacity(len);
        let bytes = unsafe {
            slice::from_raw_parts_mut(
                vec.as_mut_ptr() as *mut MaybeUninit<u8>,
                len * mem::size_of::<T>(),
            )
        };
        self.buffer.read_uninit(self.offset, bytes)?;
        unsafe {
            vec.set_len(len);
        }
        Ok(vec)
    }
