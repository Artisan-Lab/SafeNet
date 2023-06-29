fn main() {
    let layout = std::alloc::Layout::array::<u32>(16).expect("overflow cannot happen");
    let mut vec = Vec::with_capacity(layout.size() / std::mem::size_of::<u32>());
    unsafe {
        let ptr = vec.as_mut_ptr();
        std::ptr::write(ptr, 1_000_000);
        vec.set_len(1);
    }

    assert_eq!(vec, &[1_000_000]);
    assert_eq!(vec.capacity(), 16);
}
