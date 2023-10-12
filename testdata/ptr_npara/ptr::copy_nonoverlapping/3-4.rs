pub fn pop_vertex<T: Copy>(&mut self) -> Result<T, ValidationError> {
    if std::mem::size_of::<T>() == self.vertex_buffer.vertex_size as usize
        && self.vertex_buffer.data.len() >= self.vertex_buffer.vertex_size as usize
    {
        unsafe {
            let mut v = MaybeUninit::<T>::uninit();
            std::ptr::copy_nonoverlapping(
                self.vertex_buffer.data.as_ptr().add(
                    self.vertex_buffer.data.len() - self.vertex_buffer.vertex_size as usize,
                ),
                v.as_mut_ptr() as *mut u8,
                self.vertex_buffer.vertex_size as usize,
            );
            self.vertex_buffer.data.drain(
                (self.vertex_buffer.data.len() - self.vertex_buffer.vertex_size as usize)..,
            );
            self.vertex_buffer.vertex_count -= 1;
            Ok(v.assume_init())
        }
    } else {
        Err(ValidationError::InvalidVertexSize {
            expected: self.vertex_buffer.vertex_size,
            actual: std::mem::size_of::<T>() as u8,
        })
    }
}
// https://github.com/FyroxEngine/Fyrox/blob/cb4868bc332309e36b23e11f679698f076c16583/src/scene/mesh/buffer.rs#L238