fn drop(&mut self) {
    let l = self.next_biased_index.load(Ordering::Relaxed);
    if l == MIN_SIZE {
        return;
    }
    let (last_a, last_b) = index(l - 1);
    for (a, bucket) in self.buckets.iter_mut().enumerate().rev() {
        if a < last_a {
            break;
        }
        let mut b_ptr = if let Some(nn) = NonNull::new(bucket.load(Ordering::Relaxed)) {
            nn
        } else {
            panic!("Null bucket pointer before length.  Shouldn't happen.")
        };
        let cap = bucket_capacity(a);
        let sz = if a == last_a { last_b + 1 } else { cap };
        let iv: Vec<T> = unsafe {
            let b_ptr: *mut T = (b_ptr.as_mut()).as_mut_ptr();
            Vec::from_raw_parts(b_ptr, sz, cap)
        };
        drop(iv);
        bucket.store(ptr::null_mut(), Ordering::Relaxed)
    }
}
// https://github.com/facebook/hhvm/blob/eb80592a45e22de5590ccb534065984041e1da70/hphp/hack/src/utils/intern/src/atomic_arena.rs#L444