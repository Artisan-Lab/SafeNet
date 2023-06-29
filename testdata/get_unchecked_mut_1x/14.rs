pub fn remove_entry(self) -> (K, V) {
    let n = unsafe { self.map.store.get_unchecked_mut(self.idx) };
    (n.key.clone(), n.value.take().unwrap())

}