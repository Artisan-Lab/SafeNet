/*
https://github.com/Rust-for-Linux/linux/blob/c8d1ae2cbe240789ad402c71fce78a7ea1ebdea5/rust/kernel/sync/arc.rs#L364
*/
 fn try_from(mut v: Vec<T>) -> Result<Self> {
        let value_layout = Layout::array::<T>(v.len())?;
        let layout = Layout::new::<ArcInner<()>>()
            .extend(value_layout)?
            .0
            .pad_to_align();
        // SAFETY: The layout size is guaranteed to be non-zero because `ArcInner` contains the
        // reference count.
        let ptr = NonNull::new(unsafe { alloc(layout) }).ok_or(ENOMEM)?;
        let inner =
            core::ptr::slice_from_raw_parts_mut(ptr.as_ptr() as _, v.len()) as *mut ArcInner<[T]>;

        // SAFETY: Just an FFI call that returns a `refcount_t` initialised to 1.
        let count = Opaque::new(unsafe { bindings::REFCOUNT_INIT(1) });
        // SAFETY: `inner.refcount` is writable and properly aligned.
        unsafe { core::ptr::addr_of_mut!((*inner).refcount).write(count) };
        // SAFETY: The contents of `v` as readable and properly aligned; `inner.data` is writable
        // and properly aligned. There is no overlap between the two because `inner` is a new
        // allocation.
        unsafe {
            core::ptr::copy_nonoverlapping(
                v.as_ptr(),
                core::ptr::addr_of_mut!((*inner).data) as *mut [T] as *mut T,
                v.len(),
            )
        };
        // SAFETY: We're setting the new length to zero, so it is <= to capacity, and old_len..0 is
        // an empty range (so satisfies vacuously the requirement of being initialised).
        unsafe { v.set_len(0) };
        // SAFETY: We just created `inner` with a reference count of 1, which is owned by the new
        // `Arc` object.
        Ok(unsafe { Self::from_inner(NonNull::new(inner).unwrap()) })
    }
}