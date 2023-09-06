pub fn from_vec(input: Vec<$struct_type>) -> Self {

    extern "C" fn $c_destructor_fn_name(s: &mut $struct_name) {
        let _ = unsafe { Vec::from_raw_parts(s.ptr as *mut $struct_type, s.len, s.cap) };
    }

    let ptr = input.as_ptr();
    let len = input.len();
    let cap = input.capacity();

    let _ = ::core::mem::ManuallyDrop::new(input);

    Self {
        ptr,
        len,
        cap,
        destructor: $destructor_name::External($c_destructor_fn_name),
    }
}
// https://github.com/fschutt/azul/blob/b40a2ad6e22382842ce6ccd631b94fb5a6e1742d/api/_patches/azul.rs/vec.rs#L118