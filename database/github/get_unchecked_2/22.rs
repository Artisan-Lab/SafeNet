pub fn key(&self) -> &K {
    unsafe { &self.map.store.get_unchecked(self.idx).key }
}