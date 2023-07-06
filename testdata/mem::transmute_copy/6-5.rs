fn decode_value(decoder: &mut Decoder, context: Context) -> Result<[T; N], ValidationError> {
    let elems = {
        let state = decoder.get_mut(&context);
        match state.decode_array_header::<T>() {
            Ok(header) => header.data(),
            Err(err) => return Err(err),
        }
    };
    if elems as usize != N {
        return Err(ValidationError::UnexpectedArrayHeader);
    }

    // Since we don't force Clone to be implemented on Mojom types
    // (mainly due to handles) we need to create this array as uninitialized
    // and initialize it manually.
    let mut array_uninit: [mem::MaybeUninit<T>; N] = unsafe {
        // We call assume_init() for a MaybeUninit<[MaybeUnint]> which drops the outer
        // MaybeUninit, producing a "initialized" array of MaybeUnint elements. This is
        // fine since MaybeUninit does not actually store whether it's initialized and
        // our code hereafter continues to assume the elements are not initialized yet.
        // See https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        mem::MaybeUninit::uninit().assume_init()
    };

    let mut panic_error = None;
    let mut inits = 0;
    let mut error = None;
    for elem in &mut array_uninit[..] {
        // When a panic unwinds it may try to read and drop uninitialized memory, so we
        // need to catch this. However, we pass mutable state! This could be bad as we
        // could observe a broken invariant inside of decoder and access it as usual,
        // but we do NOT access decoder here, nor do we ever unwind through one of
        // decoder's methods. Therefore, it should be safe to assert that decoder is
        // unwind safe.
        let next_context = context.clone();
        // We assert everything going into this closure is unwind safe. If anything
        // is added, PLEASE make sure it is also unwind safe...
        let result =
            panic::catch_unwind(panic::AssertUnwindSafe(|| T::decode(decoder, next_context)));
        match result {
            Ok(non_panic_value) => match non_panic_value {
                Ok(value) => elem.write(value),
                Err(err) => {
                    error = Some(err);
                    break;
                }
            },
            Err(err) => {
                panic_error = Some(err);
                break;
            }
        };
        inits += 1;
    }
    if panic_error.is_some() || error.is_some() {
        // Drop everything that was initialized
        for i in 0..inits {
            // In the previous for loop, we initialized the first `inits` array values.
            // It is safe to get a mutable reference to these.
            unsafe {
                // Drop by pointer since we can't move out of an array.
                ptr::drop_in_place(array_uninit[i].as_mut_ptr());
            }
        }
        if let Some(err) = panic_error {
            panic::resume_unwind(err);
        }
        Err(error.take().expect("Corrupted stack?"))
    } else {
        // This is safe since [T; N] and [MaybeUninit<T>; N] are
        // layout-equivalent, every value is initialized, and MaybeInit<T> never calls
        // drop on its inner value.
        //
        // Unfortunately regular transmute doesn't work: it can't handle types
        // parameterized by generic T, even though the arrays are the same size. Known
        // issue: https://github.com/rust-lang/rust/issues/47966
        let array =
            unsafe { mem::transmute_copy::<[mem::MaybeUninit<T>; N], [T; N]>(&array_uninit) };
        Ok(array)
    }
}

// https://github.com/DiversionCompany/chromium-1/blob/7b7ecfe3fe7cba6fdddcb4d9ce102e450066251c/mojo/public/rust/bindings/mojom.rs#L658