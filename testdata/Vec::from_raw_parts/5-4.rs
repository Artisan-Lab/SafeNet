fn u8_vec_to_i8_vec(vec: Vec<u8>) -> Vec<i8> {
    // SAFETY: Layouts of u8 and i8 are the same and we're being careful not to drop
    // the original vec after calling Vec::from_raw_parts.
    unsafe {
        let mut vec = ManuallyDrop::new(vec);
        Vec::from_raw_parts(vec.as_mut_ptr() as *mut i8, vec.len(), vec.capacity())
    }
}