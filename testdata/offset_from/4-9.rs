pub(crate) unsafe fn get_range(self, idx: usize) -> Range<usize> {
    let range = self.get_things_range(idx);
    let offset = (self.things as *const [u8] as *const u8)
        .offset_from(self.entire_slice as *const [u8] as *const u8)
        as usize;
    range.start + offset..range.end + offset
}
//https://github.com/unicode-org/icu4x/blob/67ff68456d8ec87ceef26b81c36099264673520f/utils/zerovec/src/varzerovec/components.rs#L305