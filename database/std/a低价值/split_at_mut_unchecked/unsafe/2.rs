#![feature(raw_slice_split)]

let mut v = [1, 0, 3, 0, 5, 6];
// scoped to restrict the lifetime of the borrows
unsafe {
    let ptr = &mut v as *mut [_];
    let (left, right) = ptr.split_at_mut_unchecked(2);
    assert_eq!(&*left, [1, 0]);
    assert_eq!(&*right, [3, 0, 5, 6]);
    (&mut *left)[1] = 2;
    (&mut *right)[1] = 4;
}
assert_eq!(v, [1, 2, 3, 4, 5, 6]);