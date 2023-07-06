fn main() {
    let ptr: *mut i32 = &mut 0;
    let ref_transmuted: &mut i32 = unsafe {
        std::mem::transmute_copy(&ptr)
    };
    // Use a reborrow instead
    //let ref_casted = unsafe { &mut *ptr };
}
