pub fn copy_to(&self, dest: Address, size: usize) {
    unsafe {
        ptr::copy(
            self as *const Obj as *const u8,
            dest.to_mut_ptr::<u8>(),
            size,
        );
    }
}
// https://github.com/dinfuehr/dora/blob/687efd630c4e1c30104d72ed6d47ec578119e4d8/dora-runtime/src/object.rs#L310