#![feature(slice_ptr_get, nonnull_slice_from_raw_parts)]
use std::ptr::NonNull;

let x = &mut [1, 2, 4];
let x = NonNull::slice_from_raw_parts(NonNull::new(x.as_mut_ptr()).unwrap(), x.len());

unsafe {
    assert_eq!(x.get_unchecked_mut(1).as_ptr(), x.as_non_null_ptr().as_ptr().add(1));
}