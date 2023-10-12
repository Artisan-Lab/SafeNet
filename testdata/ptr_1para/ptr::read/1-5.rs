unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    let block = {
        if layout.align() <= MIN_ALIGN {
            ptr
        } else {
            // The location of the start of the block is stored in the padding before `ptr`.

            // SAFETY: Because of the contract of `System`, `ptr` is guaranteed to be non-null
            // and have a header readable directly before it.
            unsafe { ptr::read((ptr as *mut Header).sub(1)).0 }
        }
    };

    // SAFETY: because `ptr` has been successfully allocated with this allocator,
    // `HEAP` must have been successfully initialized.
    let heap = unsafe { get_process_heap() };

    // SAFETY: `heap` is a non-null handle returned by `GetProcessHeap`,
    // `block` is a pointer to the start of an allocated block.
    unsafe { HeapFree(heap, 0, block as c::LPVOID) };
}

// https://github.com/glaubitz/rust/blob/1f5768bc67ecb025342770e14e03699699965706/library/std/src/sys/windows/alloc.rs#L217
