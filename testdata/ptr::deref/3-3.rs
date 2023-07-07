fn get_type<'a, T>(&self, offset: usize) -> Option<(&'a T, usize)> {
    let (data, next) = self.get_slice(offset, mem::size_of::<T>())?;
    let ptr: *const T = data.as_ptr() as *const T;
    //UNSAFE: The cast is safe because the slice is aligned and fits into the memory
    //and the lifetime of he &T is tied to self, which holds the underlying memory map
    Some((unsafe { &*ptr }, next))
}
//https://github.com/ChorusOne/sohlarna/blob/6e2d930ac6b8a7fc93ba3c18edf5f721702b7703/runtime/src/append_vec.rs#L346