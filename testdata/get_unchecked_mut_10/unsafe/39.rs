#[inline]
pub fn get_mut(&mut self) -> &mut V {
    unsafe {
        if let Node { value: Some(v), .. } = self.map.store.get_unchecked_mut(self.idx) {
            v
        } else {
            unreachable!()
        }
    }
}