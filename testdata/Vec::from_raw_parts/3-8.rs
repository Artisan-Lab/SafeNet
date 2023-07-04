fn new_node<const N: usize>() -> Node<T, N, Self> {
    let items = {
        let layout = std::alloc::Layout::array::<NodeItem<T>>(N).unwrap();
        unsafe {
            Vec::from_raw_parts(std::alloc::alloc_zeroed(layout) as *mut _, N, N)
        }
    };
    let items = UnsafeCell::new(items);
    let next = default();
    let _allocation_behavior = ZST();
    Node { _allocation_behavior, items, next }
}

// https://github.com/enso-org/enso/blob/b243a5f5292bf01c810cd37ecd2f04ee751c1601/lib/rust/data-structures/src/unrolled_linked_list.rs#L667
