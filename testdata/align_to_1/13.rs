fn slice_u64_to_i8(input: &[u64]) -> &[i8] {
    let (head, body, tail) = unsafe { input.align_to::<i8>() };

    // Ensure that alignment is correct
    assert!(head.is_empty());
    assert!(tail.is_empty());

    body
    
}