pub fn str(&self, row: usize, table: usize, column: usize) -> &str {
    let offset = self.strings + self.usize(row, table, column);
    let bytes = &self.bytes[offset..];
    let nul_pos = bytes.iter().position(|&c| c == 0).expect("expected null-terminated C-string");
    std::str::from_utf8(&bytes[..nul_pos]).expect("expected valid utf-8 C-string")
}