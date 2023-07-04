let mut val_buf = MutableBuffer::new(num_bytes).with_bitset(num_bytes, false);
let data = unsafe {
    std::slice::from_raw_parts_mut(val_buf.raw_data_mut(), val_buf.capacity())
};
