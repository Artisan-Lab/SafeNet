fn process_raw(raw1: *mut i32, raw2: *mut i32) {
    unsafe {
        let distance = raw1.offset_from(raw2);
    }
}