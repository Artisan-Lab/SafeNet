use std::alloc::{alloc, Layout};

fn create_box_with_layout<T>(value: T) -> Box<T> {
    let layout = Layout::new::<T>();
    let ptr = unsafe {
        let raw_ptr = alloc(layout) as *mut T;
        std::ptr::write(raw_ptr, value);
        NonNull::new_unchecked(raw_ptr)
    };
    unsafe { Box::from_raw(ptr.as_ptr()) }
}