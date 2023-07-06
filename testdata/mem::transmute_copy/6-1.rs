fn decode_value(decoder: &mut Decoder, context: Context) -> Result<[T; N], ValidationError> {
    let state = decoder.get_mut(&context);
    let len = state.decode_array_header::<T>()?.data();
    if len as usize != N {
        return Err(ValidationError::UnexpectedArrayHeader);
    }

    // Mojom objects are encoded and decoded "by-value", since they can
    // transfer handles which are owned. Since there is no default value for
    // T, we must initialize the array element-by-element.
    let mut array_uninit: [mem::MaybeUninit<T>; N] = unsafe {
        // We call assume_init() for a MaybeUninit<[MaybeUnint]> which drops the outer
        // MaybeUninit, producing a "initialized" array of MaybeUnint elements. This is
        // fine since MaybeUninit does not actually store whether it's initialized and
        // our code hereafter continues to assume the elements are not initialized yet.
        // See https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        mem::MaybeUninit::uninit().assume_init()
    };

    for elem in &mut array_uninit {
        let next_context = context.clone();

        // This is unwind-safe. If `T::decode` panics, we simply leak the
        // previously-decoded array elements. Since it's an array of
        // `MaybeUninit<T>` drop will not be called.
        //
        // Leaking values is OK since panics are intended to be
        // unrecoverable.
        elem.write(T::decode(decoder, next_context)?);
    }

    // SAFETY:
    // * Every `MaybeUninit<T>` element of `array_uninit` has been initialized,
    //   since in our loop above looped over every element and wrote a valid value.
    // * Transmute from `[MaybeUninit<T>; N]` to `[T; N]` is safe if all elements
    //   are initialized.
    //
    // Unfortunately regular transmute doesn't work: it can't handle types
    // parameterized by generic T, even though the arrays are the same size.
    // Known issue: https://github.com/rust-lang/rust/issues/47966
    let array =
        unsafe { mem::transmute_copy::<[mem::MaybeUninit<T>; N], [T; N]>(&array_uninit) };
    Ok(array)
}

// https://github.com/nwjs/chromium.src/blob/21945e5c163533b01a44214b663a9bf615c31ab5/mojo/public/rust/bindings/mojom.rs#L556