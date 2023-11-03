fn validate_guard_region(region: &GuestMmapRegion) {
    let page_size = get_page_size().unwrap();

    // Check that the created range allows us to write inside it
    let addr = region.as_ptr();

    unsafe {
        std::ptr::write(addr, 0xFF);
        assert_eq!(std::ptr::read(addr), 0xFF);
    }

    // Try a read/write operation against the left guard border of the range
    let left_border = (addr as usize - page_size) as *mut u8;
    fork_and_run(&|| AddrOp::Read.apply_on_addr(left_border), true);
    fork_and_run(&|| AddrOp::Write.apply_on_addr(left_border), true);

    // Try a read/write operation against the right guard border of the range
    let right_border = (addr as usize + region.size()) as *mut u8;
    fork_and_run(&|| AddrOp::Read.apply_on_addr(right_border), true);
    fork_and_run(&|| AddrOp::Write.apply_on_addr(right_border), true);
}

// https://github.com/firecracker-microvm/firecracker/blob/5a6f28dea0fcb5c1cc61e1cb5a5cf4414eb08778/src/utils/src/vm_memory.rs#L472