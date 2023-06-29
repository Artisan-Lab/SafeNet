let mut val_buf = MutableBuffer::new(num_bytes).with_bitset(num_bytes, false);
let data = val_buf.as_slice_mut();
