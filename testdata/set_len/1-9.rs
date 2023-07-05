/*
https://github.com/Tantalus13A98B5F/tvm/blob/0e81cf18ad41b0050920e64e9ab4cf884b741ef6/rust/tvm-rt/src/ndarray.rs#L275
*/
 pub fn to_vec<T>(&self) -> Result<Vec<T>, NDArrayError> {
        let n = self.size() / size_of::<T>();
        let mut vec: Vec<T> = Vec::with_capacity(n);

        let ptr = vec.as_mut_ptr();
        let slice = unsafe { slice::from_raw_parts_mut(ptr, n) };
        self.copy_to_buffer(slice);

        unsafe { vec.set_len(n) };
        Ok(vec)
    }