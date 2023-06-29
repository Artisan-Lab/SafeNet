pub(crate) fn increment_by<T>(slice: &mut &[T], amount: usize) {

    *slice = &core::mem::replace(slice, &[])[amount..]
    // 直接对结构体中的内部字段进行了引用，并使用了 mem::replace 函数来替换原先的空切片
}