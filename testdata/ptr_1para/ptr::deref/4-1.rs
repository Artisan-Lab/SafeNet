pub fn follow_cast_ref<'a, T: Sized + 'a>(buf: &'a [u8], loc: usize) -> &'a T {
    let sz = size_of::<T>();
    let buf = &buf[loc..loc + sz];
    let ptr = buf.as_ptr() as *const T;
    unsafe { &*ptr }
}
//https://github.com/rw/flatbuffers/blob/615885e889c92306f16b963fc4f88d1a447debf4/rust/flatbuffers/src/vector.rs#L94