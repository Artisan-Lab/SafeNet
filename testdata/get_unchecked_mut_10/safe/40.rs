#[inline]
pub fn into_mut(self) -> &'a mut V {
    match self.pair {
        Some(pair) => &mut pair.value,
        None => unreachable!(),
    }
}