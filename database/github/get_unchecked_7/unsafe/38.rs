#[inline]
pub fn get(&self) -> &V {
    unsafe { &self.map.store.get_unchecked(self.idx).Value }
}