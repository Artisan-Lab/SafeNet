unsafe fn new_at(
    ptr: NonNull<ComponentInstance>,
    alloc_size: usize,
    offsets: VMComponentOffsets<HostPtr>,
    runtime_info: Arc<dyn ComponentRuntimeInfo>,
    store: *mut dyn Store,
) {
    assert!(alloc_size >= Self::alloc_layout(&offsets).size());

    ptr::write(
        ptr.as_ptr(),
        ComponentInstance {
            offsets,
            vmctx_self_reference: SendSyncPtr::new(
                NonNull::new(
                    ptr.as_ptr()
                        .cast::<u8>()
                        .add(mem::size_of::<ComponentInstance>())
                        .cast(),
                )
                .unwrap(),
            ),
            runtime_info,
            vmctx: VMComponentContext {
                _marker: marker::PhantomPinned,
            },
        },
    );

    (*ptr.as_ptr()).initialize_vmctx(store);
}