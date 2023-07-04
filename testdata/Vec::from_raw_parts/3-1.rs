fn to_shmem(&self, builder: &mut SharedMemoryBuilder) -> Result<Self> {
    unsafe {
        let dest = to_shmem_slice(self.iter(), builder)? as *mut T;
        let dest_vec = Vec::from_raw_parts(dest, self.len(), self.len());
        Ok(ManuallyDrop::new(dest_vec))
    }
}

// https://github.com/servo/servo/blob/66abb1dfc49ab1067b7a3d7bc2d40b772f949bcd/components/to_shmem/lib.rs#L390