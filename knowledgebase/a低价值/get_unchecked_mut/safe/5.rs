let x = &mut [1, 2, 4];
let x_ptr = NonNull::new(x.as_mut_ptr()).unwrap();
let x_slice = x_ptr.as_slice_mut(x.len());

if let Some(elem) = x_slice.get_mut(1) {
    let elem_ptr = elem as *mut i32;
    let expected_ptr = x_ptr.as_ptr().add(1);
    assert_eq!(elem_ptr, expected_ptr);
} else {
    // handle index out of bounds error
}
