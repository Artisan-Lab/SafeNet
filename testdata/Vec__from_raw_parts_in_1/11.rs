pub fn into_vec<T, A: Allocator>(b: Box<[T], A>) -> Vec<T, A> {
    unsafe {
        let len = b.len();
        let (b, alloc) = Box::into_raw_with_allocator(b);
        Vec::from_raw_parts_in(b as *mut T, len, len, alloc)
    }
}
/*
https://github.com/OpenCloudOS/OpenCloudOS-Kernel/blob/96d65459c0e883e4a1a6fb7a2ad5f05b585219a5/rust/alloc/slice.rs#L172
*/