#[inline]
pub fn remove_entry(self) -> (K, V) {

    match self.pair.take() {
        Some(pair) => (pair.key, pair.value),
        None => unreachable!(),
    }
}