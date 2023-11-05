pub fn pop_vertex<T>(&mut self) -> Result<T, ValidationError>
    where
        T: VertexTrait,
    {
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
                let range =
                    (self.vertex_buffer.data.len() - self.vertex_buffer.vertex_size as usize)..;
                self.vertex_buffer.data.drain(range);
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

    // https://github.com/FyroxEngine/Fyrox/blob/a5144485eb2b28b8a68beaf35f860bd03a0ea5b1/src/scene/mesh/buffer.rs#L408