pub(crate) fn increment_by<T>(slice: &mut &[T], amount: usize) {
    let lifetime_hack = unsafe {
        // 先切片中的前 amount 个元素移除
        let slice_ptr = slice.as_ptr();
        ::std::slice::from_raw_parts(slice_ptr, slice.len())
        // 使用了 from_raw_parts 函数，将一个指针和长度转换为切片
    };
    *slice = &lifetime_hack[amount..]

}