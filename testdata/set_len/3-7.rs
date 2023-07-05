/*
https://github.com/facebook/opaque-ke/blob/4487f11f873187fb86b09fab95ba2bf048bc90ea/src/util.rs#L37
*/
pub(crate) fn drop_manually<T: Sized>(value: &mut T) {
    use std::{mem, ptr, vec};

    assert!(mem::needs_drop::<T>());
    let mut test_holder = vec![value];
    let ptr = &mut *test_holder[0] as *mut T;

    unsafe {
        test_holder.set_len(0);
        ptr::drop_in_place(ptr);
    }

    assert_eq!(test_holder.capacity(), 1);
}