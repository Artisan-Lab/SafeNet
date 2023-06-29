impl AccessBlock {
    fn new(item: CacheAccess) -> AccessBlock {
        let mut ret = AccessBlock {
            len: AtomicUsize::new(1),
            block: unsafe { MaybeUninit::zeroed().assume_init() },
            next: AtomicPtr::default(),
        };
    }
}
/*
https://github.com/spacejam/sled/blob/69294e59c718289ab3cb6bd03ac3b9e1e072a1e7/src/lru.rs#L56 
*/
