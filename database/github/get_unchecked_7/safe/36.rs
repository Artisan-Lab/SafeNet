#[inline]
pub fn key(&self) -> &K {

    match &*self.pair {
        Some(pair) => &pair.key,
        None => unreachable!(),
    }
}