pub fn into_vec<T, A: Allocator>(b: Box<[T], A>) -> Vec<T, A> {
    unsafe {
        let len = b.len();
        let (b, alloc) = Box::into_raw_with_allocator(b);
        Vec::from_raw_parts_in(b as *mut T, len, len, alloc)
    }
}           

// https://github.com/Rust-for-Linux/linux/blob/c8d1ae2cbe240789ad402c71fce78a7ea1ebdea5/rust/alloc/slice.rs#L106