fn process_raw(ref1:&mut i32, ref2: &mut i32) {
    let raw1 = ref1 as *mut i32;
    let raw2 = ref2 as *mut i32;
    unsafe {
        let distance = raw1.offset_from(raw2);
    }
}