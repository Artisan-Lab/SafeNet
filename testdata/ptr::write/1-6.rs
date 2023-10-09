unsafe fn write_to_ptr(string: &str, ptr: *mut u8) {
    ptr::write(ptr as *mut _, AtomHeader::build_with(string.len() as u64));
    let str_ptr = (ptr as usize + mem::size_of::<AtomHeader>()) as *mut u8;
    ptr::copy_nonoverlapping(string.as_ptr(), str_ptr as *mut u8, string.len());
}
// https://github.com/mthom/scryer-prolog/blob/b5b45dde9d748644e16cefac74deae66fa50f26e/src/atom_table.rs#L219