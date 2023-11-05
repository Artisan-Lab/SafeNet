pub(crate) fn get_cached_magazine(
    &self,
    cache: &mut LocalMagazineCache,
) -> Option<PopMagazine> {
    // Pop from partial magazines first, because we'd prefer to
    // have 0 partial mag.
    let ret = cache
        .steal_full()
        .or_else(|| self.partial_mags.try_pop())
        .or_else(|| self.full_mags.pop())?;

    if self.zero_init {
        for allocation in ret.get_populated() {
            unsafe {
                let alloc = &*allocation.as_ptr();
                std::ptr::write_bytes(alloc.get().as_ptr() as *mut u8, 0, self.layout.size());
            }
        }
    }
}
// https://github.com/backtrace-labs/slitter/blob/0aa75ef2706e290c24722b5d62a241c4ac392c9e/src/magazine.rs#L278