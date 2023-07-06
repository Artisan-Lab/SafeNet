pub unsafe fn unsync_store(&self, t: T) {
    debug_assert!(AtomicCell::<Option<T>>::is_lock_free());
    let this = self.inner.as_ptr();
    let a = unsafe { &*(this as *const _ as *const AtomicUsize) };
    unsafe { a.store(std::mem::transmute_copy(&t), Ordering::SeqCst) };
    std::mem::forget(t);
}
// https://github.com/search?q=mem%3A%3Atransmute_copy+NOT+%22fn+transmute_copy%22+Language%3ARust&type=code