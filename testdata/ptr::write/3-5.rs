unsafe fn try_allocate_for_layout(
    value_layout: Layout,
    allocate: impl FnOnce(Layout) -> Result<NonNull<[u8]>, AllocError>,
    mem_to_arcinner: impl FnOnce(*mut u8) -> *mut ArcInner<T>,
) -> Result<*mut ArcInner<T>, AllocError> {
    let layout = arcinner_layout_for_value_layout(value_layout);

    let ptr = allocate(layout)?;

    // Initialize the ArcInner
    let inner = mem_to_arcinner(ptr.as_non_null_ptr().as_ptr());
    debug_assert_eq!(unsafe { Layout::for_value(&*inner) }, layout);

    unsafe {
        ptr::write(&mut (*inner).strong, atomic::AtomicUsize::new(1));
        ptr::write(&mut (*inner).weak, atomic::AtomicUsize::new(1));
    }

    Ok(inner)
}