/*
https://github.com/facebook/hhvm/blob/eb80592a45e22de5590ccb534065984041e1da70/hphp/hack/src/shmffi/ocaml_blob.rs#L141
*/

pub unsafe fn to_ocaml_value(&self) -> usize {
        if !self.header.is_serialized() {
            caml_alloc_initialized_string(self.header.buffer_size(), self.data.as_ptr())
        } else if !self.header.is_compressed() {
            caml_input_value_from_block(self.data.as_ptr(), self.header.buffer_size())
        } else {
            let mut data: Vec<u8> = Vec::with_capacity(self.header.uncompressed_size());
            let uncompressed_size = liblz4::LZ4_decompress_safe(
                self.data.as_ptr() as *const libc::c_char,
                data.as_mut_ptr() as *mut libc::c_char,
                self.header.buffer_size().try_into().unwrap(),
                self.header.uncompressed_size().try_into().unwrap(),
            );
            let uncompressed_size: usize = uncompressed_size.try_into().unwrap();
            assert!(self.header.uncompressed_size() == uncompressed_size);
            // SAFETY: `LZ4_decompress_safe` should have initialized
            // `uncompressed_size` bytes; we assert above that
            // `uncompressed_size` is equal to the capacity we set
            data.set_len(uncompressed_size);

            caml_input_value_from_block(data.as_ptr(), data.len())
        }
    }