use std::ptr::NonNull;
pub fn insert(
    &self,
    key: K,
    layout: Option<Layout>,
    evictable: bool,
    value: impl FnOnce(&mut [u8]) -> V,
) -> bool {
    let empty_slice: &mut [u8] = &mut [];
    let mut shard = self.shard_for_writing(&key);
    let ptr_opt = match layout {
        None => Some(NonNull::new(empty_slice as *mut [u8]).unwrap()),
        Some(layout) => {
            if evictable
                && layout.align() + layout.size() > self.max_evictable_bytes_per_shard / 2
            {
                // Requested memory is too large, do not allocate
                None
            } else if evictable {
                match shard.alloc_evictable.allocate(layout) {
                    Ok(ptr) => Some(ptr),
                    Err(AllocError) => {
                        // The allocator is full, empty the shard and try again.
                        // This time allocation MUST succeed.
                        Self::empty_shard(&mut shard);
                        Some(shard.alloc_evictable.allocate(layout).unwrap())
                    }
                }
            } else {
                Some(shard.alloc_non_evictable.allocate(layout).unwrap())
            }
        }
    };
    if let Some(mut ptr) = ptr_opt {
        // Safety: we are the only ones with access to the allocated chunk
        let buffer = unsafe { ptr.as_mut() };
        let v = value(buffer);
        assert!(v.points_to_evictable_data() == evictable);
        shard.map.insert(key, v);
        return true;
    }
    false
}