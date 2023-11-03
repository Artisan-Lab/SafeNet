fn new<T>(header: H, value: T) -> WithHeader<H> {
    let value_layout = Layout::new::<T>();
    let Ok((layout, value_offset)) = Self::alloc_layout(value_layout) else {
        // We pass an empty layout here because we do not know which layout caused the
        // arithmetic overflow in `Layout::extend` and `handle_alloc_error` takes `Layout` as
        // its argument rather than `Result<Layout, LayoutError>`, also this function has been
        // stable since 1.28 ._.
        //
        // On the other hand, look at this gorgeous turbofish!
        alloc::handle_alloc_error(Layout::new::<()>());
    };

    unsafe {
        // Note: It's UB to pass a layout with a zero size to `alloc::alloc`, so
        // we use `layout.dangling()` for this case, which should have a valid
        // alignment for both `T` and `H`.
        let ptr = if layout.size() == 0 {
            // Some paranoia checking, mostly so that the ThinBox tests are
            // more able to catch issues.
            debug_assert!(
                value_offset == 0 && mem::size_of::<T>() == 0 && mem::size_of::<H>() == 0
            );
            layout.dangling()
        } else {
            let ptr = alloc::alloc(layout);
            if ptr.is_null() {
                alloc::handle_alloc_error(layout);
            }
            // Safety:
            // - The size is at least `aligned_header_size`.
            let ptr = ptr.add(value_offset) as *mut _;

            NonNull::new_unchecked(ptr)
        };

        let result = WithHeader(ptr, PhantomData);
        ptr::write(result.header(), header);
        ptr::write(result.value().cast(), value);

        result
    }
}   
// https://github.com/Rust-for-Linux/linux/blob/c8d1ae2cbe240789ad402c71fce78a7ea1ebdea5/rust/alloc/boxed/thin.rs#L224