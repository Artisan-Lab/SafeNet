unsafe fn finalize(ptr: *const Self) {
    let ptr = ptr as *mut Self;

    // Call destructors: drop the key and the value.
    ptr::drop_in_place(&mut (*ptr).key);
    ptr::drop_in_place(&mut (*ptr).value);

    // Finally, deallocate the memory occupied by the node.
    Node::dealloc(ptr);
}
// https://github.com/crossbeam-rs/crossbeam/blob/ce31c18607c44d3d07fc3618f981e858b35e3828/crossbeam-skiplist/src/base.rs#L256