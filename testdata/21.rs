use std::alloc::{alloc, Layout};

fn allocate_and_wrap<T>(value: T) -> Box<T> {
    let layout = Layout::new::<T>();
    let ptr = unsafe { alloc(layout) as *mut T };

    if ptr.is_null() {
        panic!("Memory allocation failed");
    }

    unsafe {
        ptr.write(value);
        Box::from_raw(NonNull::new_unchecked(ptr))
    }
}
