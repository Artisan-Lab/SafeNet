fn i8_vec_to_u8_vec(vec: Vec<i8>) -> Vec<u8> {
    // SAFETY: Layouts of u8 and i8 are the same and we're being careful not to drop
    // the original vec after calling Vec::from_raw_parts.
    unsafe {
        let mut vec = ManuallyDrop::new(vec);
        Vec::from_raw_parts(vec.as_mut_ptr() as *mut u8, vec.len(), vec.capacity())
    }
}