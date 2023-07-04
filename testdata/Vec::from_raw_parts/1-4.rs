pub extern "C" fn crsql_free_unpacked_values(values: RawVec) {
    let unpacked = unsafe {
        Vec::from_raw_parts(
            values.ptr as *mut ColumnValue,
            values.len as usize,
            values.cap as usize,
        )
    };
    drop(unpacked);
}

// https://github.com/vlcn-io/cr-sqlite/blob/3b880776fad3b7878ccad67c2f5d953971ffccc8/core/rs/core/src/pack_columns.rs#L224