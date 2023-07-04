pub fn str(&self, row: usize, table: usize, column: usize) -> &str {
    let offset = self.strings + self.usize(row, table, column);

    unsafe {
        let len = strlen(self.bytes.as_ptr().add(offset));
        std::str::from_utf8_unchecked(&self.bytes[offset..offset + len])
    }
}