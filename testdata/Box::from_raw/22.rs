use std::alloc::{alloc, Layout};
fn allocate_and_box<T>(value: T) -> Box<T> {
    let layout = Layout::new::<T>();
    let raw_ptr = unsafe { alloc(layout) as *mut T };
    if raw_ptr.is_null() {
        panic!("Failed to allocate memory");
    }
    let non_null = NonNull::new(raw_ptr).expect("NonNull conversion failed");
    let boxed = unsafe { Box::from_raw(non_null.as_ptr()) };
    *boxed = value;
    boxed
}