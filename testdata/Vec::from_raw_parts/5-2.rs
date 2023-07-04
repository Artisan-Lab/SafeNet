unsafe fn get_unchecked_str(cp: *mut u8, start: *mut u8) -> String {
    let len = cp as usize - start as usize;
    let part = Vec::from_raw_parts(start, len, len);
    let tmp = String::from_utf8_unchecked(part.clone());
    ::std::mem::forget(part);
    tmp
}

// https://github.com/dalance/procs/blob/f17e855b74f8dafdbd4b1f0a4299077ce2c1603d/src/process/macos.rs#L152
