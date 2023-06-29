#[inline]
pub fn insert(self, value: V) -> &'a mut V
where
    K: Eq + Hash,
{
    let i = self.map.store.len();
    self.map.insert(self.key, value);
    if let Node { value: Some(v), .. } = unsafe { self.map.store.get_unchecked_mut(i) } {
        v
    } else {
        unreachable!()
    }
}