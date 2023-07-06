pub fn by_value<T: Variant + Clone>(data: &mut Dynamic) -> T {
    if TypeId::of::<T>() == TypeId::of::<&str>() {
        // If T is `&str`, data must be `ImmutableString`, so map directly to it
        data.flatten_in_place();
        let ref_str = data.as_str_ref().expect("&str");
        // SAFETY: We already checked that `T` is `&str`, so it is safe to cast here.
        return unsafe { mem::transmute_copy::<_, T>(&ref_str) };
    }
    if TypeId::of::<T>() == TypeId::of::<String>() {
        // If T is `String`, data must be `ImmutableString`, so map directly to it
        return reify!(mem::take(data).into_string().expect("`ImmutableString`") => T);
    }

    // We consume the argument and then replace it with () - the argument is not supposed to be used again.
    // This way, we avoid having to clone the argument again, because it is already a clone when passed here.
    mem::take(data).cast::<T>()
}

// https://github.com/azjezz/rhai/blob/0f4df9c4e70b5de686e0f94f5c139f5c1bce46c9/src/func/register.rs#L51