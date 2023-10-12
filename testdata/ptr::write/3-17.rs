unsafe fn fill<I: Iterator<Item = T>>(&mut self, replace_with: &mut I) -> bool {
    let vec = unsafe { self.vec.as_mut() };
    let range_start = vec.len;
    let range_end = self.tail_start;
    let range_slice = unsafe {
        slice::from_raw_parts_mut(vec.as_mut_ptr().add(range_start), range_end - range_start)
    };

    for place in range_slice {
        if let Some(new_item) = replace_with.next() {
            unsafe { ptr::write(place, new_item) };
            vec.len += 1;
        } else {
            return false;
        }
    }
    true
}
