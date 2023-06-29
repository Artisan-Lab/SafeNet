struct MyStruct<T> {
    data: Vec<T>,
}

impl<T> MyStruct<T> {
    pub fn key(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}
