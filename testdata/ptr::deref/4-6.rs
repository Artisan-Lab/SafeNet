fn push(&self, item: CacheAccess) -> bool {
    loop {
        debug_delay();
        let head = self.writing.load(Ordering::Acquire);
        let block = unsafe { &*head };

        debug_delay();
        let offset = block.len.fetch_add(1, Ordering::Acquire);

        if offset < MAX_QUEUE_ITEMS {
            let item_u64: u64 = item.into();
            assert_ne!(item_u64, 0);
            debug_delay();
            unsafe {
                block
                    .block
                    .get_unchecked(offset)
                    .store(item_u64, Ordering::Release);
            }
            return false;
        } else {
            // install new writer
            let new = Box::into_raw(Box::new(AccessBlock::new(item)));
            debug_delay();
            let res = self.writing.compare_exchange(
                head,
                new,
                Ordering::AcqRel,
                Ordering::Acquire,
            );
            if res.is_err() {
                // we lost the CAS, free the new item that was
                // never published to other threads
                unsafe {
                    drop(Box::from_raw(new));
                }
                continue;
            }

            // push the now-full item to the full list for future
            // consumption
            let mut ret;
            let mut full_list_ptr = self.full_list.load(Ordering::Acquire);
            while {
                // we loop because maybe other threads are pushing stuff too
                block.next.store(full_list_ptr, Ordering::Release);
                debug_delay();
                ret = self.full_list.compare_exchange(
                    full_list_ptr,
                    head,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                );
                ret.is_err()
            } {
                full_list_ptr = ret.unwrap_err();
            }
            return true;
        }
    }
}
//https://github.com/spacejam/sled/blob/69294e59c718289ab3cb6bd03ac3b9e1e072a1e7/src/lru.rs#L79