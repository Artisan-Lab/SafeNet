pub fn decode_variable_primitive_vec<R: io::Read, T: RosMsg>(mut r: R) -> io::Result<Vec<T>> {
    let num_elements = u32::decode(r.by_ref())? as usize;
    let num_bytes = num_elements * std::mem::size_of::<T>();
    let mut buf = Vec::<T>::with_capacity(num_elements);

    let buf_ptr = buf.as_mut_ptr();
    let read_buf = unsafe { std::slice::from_raw_parts_mut(buf_ptr as *mut u8, num_bytes) };
    r.read_exact(read_buf)?;

    std::mem::forget(buf);
    Ok(unsafe { Vec::from_raw_parts(buf_ptr, num_elements, num_elements) })
}

// https://github.com/adnanademovic/rosrust/blob/00fe62491ea76ae6908bac7b79eb1bf25fc6f64e/rosrust/src/rosmsg.rs#L235