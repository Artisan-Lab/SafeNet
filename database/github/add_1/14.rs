pub fn pick3_mut(&mut self, a: I, b: I, c: I) -> (&mut T, &mut T, &mut T) {
    let (ai, bi, ci) = (a.index(), b.index(), c.index());
    assert!(ai != bi && bi != ci && ci != ai);
    let len = self.raw.len();
    assert!(ai < len && bi < len && ci < len);
    let ptr = self.raw.as_mut_ptr();
    unsafe { (&mut *ptr.add(ai), &mut *ptr.add(bi), &mut *ptr.add(ci)) }
}

// https://github.com/esp-rs/rust/blob/f112def2207779c024b9ad42099077bb5c4c8998/compiler/rustc_index/src/vec.rs#L411