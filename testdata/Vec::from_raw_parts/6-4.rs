pub fn cast_vec<T: Copy>(mut vec: Vec<T>) -> Vec<u8> {
    let len = std::mem::size_of::<T>() * vec.len();
    let cap = std::mem::size_of::<T>() * vec.capacity();
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec);
    unsafe { Vec::from_raw_parts(ptr as _, len, cap) }
}

// https://github.com/amethyst/rendy/blob/8e3054a075b545bc1b030fa0d81c62a0b33d3740/core/src/casts.rs#L12