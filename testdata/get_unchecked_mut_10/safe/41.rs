pub fn insert(self, value: V) -> &'a mut V {
    *self.pair = Some(KeyValue { key: self.key, value });

    match self.pair {
        Some(pair) => &mut pair.value,
        None => unreachable!(),
    }
}