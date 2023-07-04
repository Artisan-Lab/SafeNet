/*
    From: https://github.com/servo/servo/blob/f034eb60323c1501ba00e86cb7c2de243f41b56c/components/to_shmem/lib.rs#L286
*/

fn to_shmem(&self, builder: &mut SharedMemoryBuilder) -> Result<Self> {
     // Reserve space for the boxed value.
    let dest: *mut T = builder.alloc_value();

    // Make a clone of the boxed value with all of its heap allocations
    // placed in the shared memory buffer.
    let value = (**self).to_shmem(builder)?;

    unsafe {
        // Copy the value into the buffer.
        ptr::write(dest, ManuallyDrop::into_inner(value));
        Ok(ManuallyDrop::new(Box::from_raw(dest)))
    }
}
