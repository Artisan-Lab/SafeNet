fn deref_aligned() {
    let mut data = [0u128; 4];
    let (prefix, data, suffix) = unsafe { data.align_to_mut() };
    assert!(prefix.is_empty());
    assert!(suffix.is_empty());

    let cases = [
        (0, 0, Ok(data.as_mut_ptr() as _)),
        (0, 1, Ok(data.as_mut_ptr() as _)),
        (0, 2, Ok(data.as_mut_ptr() as _)),
        (1, 0, Err(EFAULT)),
        (1, 1, Err(EFAULT)),
        (1, 2, Err(EFAULT)),
        (2, 0, Ok(data[2..].as_mut_ptr() as _)),
        (2, 1, Ok(data[2..].as_mut_ptr() as _)),
        (2, 2, Ok(data[2..].as_mut_ptr() as _)),
        (usize::MAX, 0, Err(EFAULT)),
        (0, usize::MAX, Err(EOVERFLOW)),
        (usize::MAX, usize::MAX, Err(EOVERFLOW)),
    ];
    test_deref(
        |data, offset, len| super::deref_aligned::<u16>(data, offset, len),
        data,
        cases,
    );
}