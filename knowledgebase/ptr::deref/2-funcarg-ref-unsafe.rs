fn process_ref(reference: &i32) {
    unsafe {
        let raw = reference as *const i32;
        let x  = *raw;
    }
}