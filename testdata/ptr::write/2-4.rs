pub fn visit_clobber<T: DummyAstNode>(t: &mut T, f: impl FnOnce(T) -> T) {
    unsafe {
        // Safe because `t` is used in a read-only fashion by `read()` before
        // being overwritten by `write()`.
        let old_t = ptr::read(t);
        let new_t =
            panic::catch_unwind(panic::AssertUnwindSafe(|| f(old_t))).unwrap_or_else(|err| {
                // Set `t` to some valid but possible meaningless value,
                // and pass the fatal error further.
                ptr::write(t, T::dummy());
                panic::resume_unwind(err);
            });
        ptr::write(t, new_t);
    }
}

// https://github.com/crablang/crab/blob/bbb1ac31cc3ab9a2148eb94675f7b3f3fb0ff285/compiler/rustc_ast/src/mut_visit.rs#L320