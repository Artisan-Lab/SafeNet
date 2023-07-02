fn swap_copy_values<T: Copy>(value1: &mut T, value2: &mut T) {
    unsafe {
        std::ptr::swap(value1, value2);
    }
}