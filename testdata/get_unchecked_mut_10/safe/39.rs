#[inline]
pub fn get_mut(&mut self) -> &mut V {
    match self.pair {
        Some(pair) => &mut pair.value,
        None => unreachable!(),
    }
}