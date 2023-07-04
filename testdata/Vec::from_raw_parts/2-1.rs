pub unsafe fn into_vec(mut self) -> Vec<u8> {
    let data = Vec::from_raw_parts(self.data, self.size, self.capacity);
    self.set_null();
    data
}

// https://github.com/trustwallet/wallet-core/blob/4443c337f6337aa7fcb76bceb3987ad6a1fbbaf2/rust/tw_memory/src/ffi/c_byte_array.rs#L65