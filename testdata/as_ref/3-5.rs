pub extern "C" fn VtabRowid(
    arg1: *mut sqlite3_vtab_cursor,
) -> ::std::os::raw::c_int {
    let ptr = std::ptr::NonNull::new(arg1 as *mut VfsStatCursor).unwrap();
    let cur: &VfsStatCursor = unsafe { ptr.as_ref() };
}
/*
https://github.com/epilys/vfsstat.rs/blob/5f3d16ef768a0994e5863991f6e9b38507b62a39/src/vtab.rs#L316
*/
