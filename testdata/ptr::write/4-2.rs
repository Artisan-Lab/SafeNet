pub(crate) fn push_back(&mut self, tasks: impl ExactSizeIterator<Item = task::Notified<T>>) {
    let len = tasks.len();
    assert!(len <= LOCAL_QUEUE_CAPACITY);

    if len == 0 {
        // Nothing to do
        return;
    }

    let head = self.inner.head.load(Acquire);
    let (steal, _) = unpack(head);

    // safety: this is the **only** thread that updates this cell.
    let mut tail = unsafe { self.inner.tail.unsync_load() };

    if tail.wrapping_sub(steal) <= (LOCAL_QUEUE_CAPACITY - len) as UnsignedShort {
        // Yes, this if condition is structured a bit weird (first block
        // does nothing, second returns an error). It is this way to match
        // `push_back_or_overflow`.
    } else {
        panic!()
    }

    for task in tasks {
        let idx = tail as usize & MASK;

        self.inner.buffer[idx].with_mut(|ptr| {
            // Write the task to the slot
            //
            // Safety: There is only one producer and the above `if`
            // condition ensures we don't touch a cell if there is a
            // value, thus no consumer.
            unsafe {
                ptr::write((*ptr).as_mut_ptr(), task);
            }
        });

        tail = tail.wrapping_add(1);
    }

    self.inner.tail.store(tail, Release);
}
// https://github.com/tokio-rs/tokio/blob/0d382faa4e557d0f6e281485cfb14a4934461c41/tokio/src/runtime/scheduler/multi_thread/queue.rs#L169