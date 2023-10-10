fn finalize(&self) {
    debug_assert_eq!(self.guard_count.get(), 0);
    debug_assert_eq!(self.handle_count.get(), 0);

    // Temporarily increment handle count. This is required so that the following call to `pin`
    // doesn't call `finalize` again.
    self.handle_count.set(1);
    unsafe {
        // Pin and move the local bag into the global queue. It's important that `push_bag`
        // doesn't defer destruction on any new garbage.
        let guard = &self.pin();
        self.global()
            .push_bag(self.bag.with_mut(|b| &mut *b), guard);
    }
    // Revert the handle count back to zero.
    self.handle_count.set(0);

    unsafe {
        // Take the reference to the `Global` out of this `Local`. Since we're not protected
        // by a guard at this time, it's crucial that the reference is read before marking the
        // `Local` as deleted.
        let collector: Collector = ptr::read(self.collector.with(|c| &*(*c)));

        // Mark this node in the linked list as deleted.
        self.entry.delete(unprotected());

        // Finally, drop the reference to the global. Note that this might be the last reference
        // to the `Global`. If so, the global data will be destroyed and all deferred functions
        // in its queue will be executed.
        drop(collector);
    }
}

// https://github.com/crossbeam-rs/crossbeam/blob/2191fcd459599b409f16e625e989a4183b9f3723/crossbeam-epoch/src/internal.rs#L525