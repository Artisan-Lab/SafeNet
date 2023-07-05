/*
https://github.com/jacques-n/arrow/blob/135a2d789c16edb897de9d38eb9a43463ab76847/rust/src/array.rs#L212
*/
fn from(data: Vec<Option<$native_ty>>) -> Self {
                const TY_SIZE: usize = mem::size_of::<$native_ty>();
                const NULL: [u8; TY_SIZE] = [0; TY_SIZE];

                let data_len = data.len() as i64;
                let size = bit_util::round_upto_multiple_of_64(data_len) as usize;
                let mut null_buffer = Vec::with_capacity(size);
                unsafe {
                    ptr::write_bytes(null_buffer.as_mut_ptr(), 0, size);
                    null_buffer.set_len(size);
                }
                let mut value_buffer: Vec<u8> = Vec::with_capacity(size * TY_SIZE);

                let mut i = 0;
                for n in data {
                    if let Some(v) = n {
                        bit_util::set_bit(&mut null_buffer[..], i);
                        value_buffer.extend_from_slice(&v.to_byte_slice());
                    } else {
                        value_buffer.extend_from_slice(&NULL);
                    }
                    i += 1;
                }

                let array_data = ArrayData::builder($data_ty)
                    .len(data_len)
                    .add_buffer(Buffer::from(Buffer::from(value_buffer)))
                    .null_bit_buffer(Buffer::from(null_buffer))
                    .build();
                PrimitiveArray::from(array_data)
            }
        }
    };