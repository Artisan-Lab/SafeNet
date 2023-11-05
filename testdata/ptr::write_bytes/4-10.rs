fn alloc<A>(arena: &A, key: &[u8], value: &[u8], height: usize) -> *mut Node
    where
        A: Arena<Stats = BasicStats>,
    {
        // Calculate node size to alloc
        let size = mem::size_of::<Node>();
        // Not all values in Node::tower will be utilized.
        let not_used = (MAX_HEIGHT - height - 1) * mem::size_of::<AtomicPtr<Node>>();
        // Space to store key/value: (key size) + key + (value size) + value
        let kv_used =
            mem::size_of::<KeySize>() + key.len() + mem::size_of::<ValueSize>() + value.len();
        // UB in fact: the `not_used` size is able to be access in a "safe" way.
        // It is guaranteed by the user to not use those memory.
        let alloc_size = size - not_used + kv_used;
        let layout =
            unsafe { Layout::from_size_align_unchecked(alloc_size, mem::align_of::<Node>()) };
        let node_ptr = arena.alloc(layout).as_ptr() as *mut Node;
        unsafe {
            let node = &mut *node_ptr;
            node.height = height;
            ptr::write_bytes(node.tower.as_mut_ptr(), 0, height + 1);
            Self::init_key_value(node, key, value);

            node_ptr
        }
    }
    // https://github.com/CeresDB/ceresdb/blob/65b133e5d184bd2d9a51ba48de3e7292bacf28df/components/skiplist/src/list.rs#L74