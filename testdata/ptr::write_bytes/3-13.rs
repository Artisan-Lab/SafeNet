unsafe {
    // FIXME: add a remap method to the page table
    let (_, _) = shim_page_table.unmap(page).unwrap();

    let mut allocator = ALLOCATOR.lock();
    let frame = allocator.allocate_frame().unwrap();
    let shim_phys_page = ShimPhysAddr::try_from(frame.start_address()).unwrap();
    let shim_virt: *mut u8 = ShimVirtAddr::from(shim_phys_page).into();
    core::ptr::write_bytes(shim_virt, 0, Size4KiB::SIZE as _);

    shim_page_table
        .map_to_with_table_flags(
            page,
            frame,
            flags,
            PageTableFlags::PRESENT
                | PageTableFlags::WRITABLE
                | PageTableFlags::USER_ACCESSIBLE,
            allocator.deref_mut(),
        )
        .unwrap()
        .flush();
};
// https://github.com/enarx/enarx/blob/e95f55f039d0702eee965dce762c3d299e0fbd6c/crates/shim-kvm/src/interrupts.rs#L336