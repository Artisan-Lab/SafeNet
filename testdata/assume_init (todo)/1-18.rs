pub unsafe fn initialize_with_hasher(
    cmap: &'shm mut MaybeUninit<Self>,
    hash_builder: S,
    file_alloc: &'shm FileAlloc,
    max_evictable_bytes_per_shard: usize,
) -> CMapRef<'shm, K, V, S> {
    let mut shard_allocs_non_evictable: [MaybeUninit<RwLock<ShardAllocControlData>>;
        NUM_SHARDS] = MaybeUninit::uninit().assume_init();
    for shard_alloc in &mut shard_allocs_non_evictable[..] {
        *shard_alloc = MaybeUninit::new(RwLock::new(ShardAllocControlData::new()));
    }
    let mut shard_allocs_evictable: [MaybeUninit<RwLock<ShardAllocControlData>>; NUM_SHARDS] =
        MaybeUninit::uninit().assume_init();
    for shard_alloc in &mut shard_allocs_evictable[..] {
        *shard_alloc = MaybeUninit::new(RwLock::new(ShardAllocControlData::new()));
    }

    let mut maps: [MaybeUninit<RwLock<Map<'shm, K, V, S>>>; NUM_SHARDS] =
        MaybeUninit::uninit().assume_init();
    for map in &mut maps[..] {
        *map = MaybeUninit::new(RwLock::new(Map::new()));
    }

}

// https://github.com/fredemmott/hhvm/blob/d3455492e558c56a52ce9f6ef5cb78d2de7a0991/hphp/hack/src/shmrs/chashmap.rs#L173