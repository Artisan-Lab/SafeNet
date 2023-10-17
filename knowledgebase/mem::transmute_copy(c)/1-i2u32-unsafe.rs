fn main() {
    let ptr = &mut 0;
    let val_transmuted: &mut u32 = unsafe {
        std::mem::transmute_copy::<&mut i32, &mut u32>(&ptr)
    };
    let val_casts: &mut u32 = unsafe {
        &mut *(ptr as *mut i32 as *mut u32)
    };
}
