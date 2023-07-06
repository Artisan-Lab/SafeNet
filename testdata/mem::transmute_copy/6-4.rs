fn deserialize<D: BinDeserializer<'de>>(mut deserializer: D) -> Result<Self> {
    // this is safe since MaybeUninit<T>'s Drop is a no-op
    let mut arr: [MaybeUninit<T>; LEN] =
        unsafe { std::mem::transmute_copy(&MaybeUninit::<T>::uninit()) };

    for idx in 0..LEN {
        arr[idx] = MaybeUninit::new(T::deserialize(&mut deserializer)?);
    }

    // this is safe since [MaybeUninit<T>; LEN] doesn't do anything on drop,
    // since MaybeUninit<T>'s Drop is a no-op
    Ok(unsafe { std::mem::transmute_copy(&arr) })
}
// https://github.com/2xsaiko/mcrestool/blob/bd0c07090e76be618158b82aa61378e0f286123b/binserde/src/serdeimpl.rs#L209