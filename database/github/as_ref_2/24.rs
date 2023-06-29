fn librustzcash_mmr_hash_node_inner<V: Version>(
    ptr: *const [u8; zcash_history::MAX_NODE_DATA_SIZE],
) -> u32 {
    let node_bytes: &[u8; zcash_history::MAX_NODE_DATA_SIZE] = unsafe {
        match ptr.as_ref() {
            Some(r) => r,
            None => return 1,
        }
    };
}

/*
https://github.com/KotoDevelopers/koto/blob/e3c9773eead914811aaa12fc0c4abedd65c38647/src/rust/src/history_ffi.rs#L246
*/