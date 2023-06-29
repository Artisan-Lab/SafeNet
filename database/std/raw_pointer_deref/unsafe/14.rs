fn main() {
    let mut arr: [MaybeUninit<i32>; 1] = [MaybeUninit::uninit(); 1];
    for elem in arr.iter_mut() {
        *elem = MaybeUninit::new(42);
    }
    let p = arr.as_mut_ptr() as *mut i32;
    let v = unsafe { *p };
}