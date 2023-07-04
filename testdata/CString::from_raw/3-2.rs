impl ToShmem for CString {
    fn to_shmem(&self, builder: &mut SharedMemoryBuilder) -> Result<Self> {
        let len = self.as_bytes_with_nul().len();

        // Reserve space for the string bytes.
        let dest: *mut c_char = builder.alloc_array(len);

        unsafe {
            // Copy the value into the buffer.
            ptr::copy(self.as_ptr(), dest, len);

            Ok(ManuallyDrop::new(CString::from_raw(dest)))
        }
    }
}

// https://github.com/servo/servo/blob/dea28b51a2ed8487b462da59937a9a4c5ee7ec10/components/to_shmem/lib.rs#L381