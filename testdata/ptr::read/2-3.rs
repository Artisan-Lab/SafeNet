pub const fn into_unique(b: Self) -> (Unique<T>, A) {
    // Box is recognized as a "unique pointer" by Stacked Borrows, but internally it is a
    // raw pointer for the type system. Turning it directly into a raw pointer would not be
    // recognized as "releasing" the unique pointer to permit aliased raw accesses,
    // so all raw pointer methods have to go through `Box::leak`. Turning *that* to a raw pointer
    // behaves correctly.
    let alloc = unsafe { ptr::read(&b.1) };
    (Unique::from(Box::leak(b)), alloc)
}
// https://github.com/STMicroelectronics/linux/blob/d33b43a4dcc4ae3cd178793c139756af77e42bde/rust/alloc/boxed.rs#L1102