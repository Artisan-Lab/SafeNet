pub unsafe extern "C" fn wr_program_cache_delete(program_cache: *mut WrProgramCache) {
    Rc::from_raw(program_cache);
}