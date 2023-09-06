// #[cfg_attr(not(test), lang = "box_free")]
// #[inline]
// This signature has to be the same as `Box`, otherwise an ICE will happen.
// When an additional parameter to `Box` is added (like `A: Allocator`), this has to be added here as
// well.
// For example if `Box` is changed to  `struct Box<T: ?Sized, A: Allocator>(Unique<T>, A)`,
// this function has to be changed to `fn box_free<T: ?Sized, A: Allocator>(Unique<T>, A)` as well.
pub(crate) unsafe fn box_free<T: ?Sized, A: Allocator>(ptr: Unique<T>, alloc: A) {
    unsafe {
        let size = size_of_val(ptr.as_ref());
        let align = min_align_of_val(ptr.as_ref());
        let layout = Layout::from_size_align_unchecked(size, align);
        alloc.deallocate(From::from(ptr.cast()), layout)
    }
}

//https://github.com/hermitcore/rust/blob/61de794664e673cb98c01274b3bc88b8a994001a/library/alloc/src/alloc.rs#L344