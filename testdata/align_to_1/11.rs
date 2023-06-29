pub fn align_u8<T>(data: &[T]) -> &[u8] {
    let (head, body, tail) = unsafe { data.align_to::<u8>() };

    assert!(head.is_empty());
    assert!(tail.is_empty());

    body
}