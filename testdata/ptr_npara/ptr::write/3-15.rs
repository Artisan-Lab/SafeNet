pub fn new_cyclic<F>(data_fn: F) -> Rc<T>
    where
        F: FnOnce(&Weak<T>) -> T,
    {
        // Construct the inner in the "uninitialized" state with a single
        // weak reference.
        let uninit_ptr: NonNull<_> = Box::leak(Box::new(RcBox {
            strong: Cell::new(0),
            weak: Cell::new(1),
            value: mem::MaybeUninit::<T>::uninit(),
        }))
        .into();

        let init_ptr: NonNull<RcBox<T>> = uninit_ptr.cast();

        let weak = Weak { ptr: init_ptr, alloc: Global };

        // It's important we don't give up ownership of the weak pointer, or
        // else the memory might be freed by the time `data_fn` returns. If
        // we really wanted to pass ownership, we could create an additional
        // weak pointer for ourselves, but this would result in additional
        // updates to the weak reference count which might not be necessary
        // otherwise.
        let data = data_fn(&weak);

        let strong = unsafe {
            let inner = init_ptr.as_ptr();
            ptr::write(ptr::addr_of_mut!((*inner).value), data);

            let prev_value = (*inner).strong.get();
            debug_assert_eq!(prev_value, 0, "No prior strong references should exist");
            (*inner).strong.set(1);

            Rc::from_inner(init_ptr)
        };

        // Strong references should collectively own a shared weak reference,
        // so don't run the destructor for our old weak reference.
        mem::forget(weak);
        strong
    }

    // https://github.com/esp-rs/rust/blob/9163a20878c13ac6ccee3fd9be70ecb20ca9a706/library/alloc/src/rc.rs#L483