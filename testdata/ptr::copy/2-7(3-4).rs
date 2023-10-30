pub fn read_to_buffer(&mut self, target: &mut [u8]) -> usize {
    let mut remaining = target.len();
    let mut read_len = 0;
    let buf = &mut self.buf;

    loop {
        if remaining == 0 {
            if self.len != 0 {
                self.len -= read_len;
            }
            break read_len;
        }

        let data = buf.pop_front();

        if data.is_none() {
            remaining = 0;
            continue;
        }

        let data = unsafe { data.unwrap_unchecked() };

        if data.len() >= remaining {
            let n = unsafe {
                let ptr = &mut target[read_len..];
                std::ptr::copy((&data[..remaining]).as_ptr(), ptr.as_mut_ptr(), remaining);
                remaining
            };

            remaining -= n;
            read_len += n;

            if data.len() != n {
                buf.push_front(data[n..].to_vec())
            }
        } else {
            let n = unsafe {
                let ptr = &mut target[read_len..];
                std::ptr::copy(data.as_ptr(), ptr.as_mut_ptr(), data.len());
                data.len()
            };

            read_len += n;
            remaining -= n;
        }
    }
}
// https://github.com/editso/fuso/blob/8086a3136e1a67045f247ffef76ca71c60b078ad/src/core/guard/buffer.rs#L83