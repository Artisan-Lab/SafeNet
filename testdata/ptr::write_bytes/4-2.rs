impl ${style_struct.gecko_struct_name} {
    #[allow(dead_code, unused_variables)]
    pub fn default(document: &structs::Document) -> Arc<Self> {
        unsafe {
            let mut result = UniqueArc::<Self>::new_uninit();
            // FIXME(bug 1595895): Zero the memory to keep valgrind happy, but
            // these looks like Valgrind false-positives at a quick glance.
            ptr::write_bytes::<Self>(result.as_mut_ptr(), 0, 1);
            Gecko_Construct_Default_${style_struct.gecko_ffi_name}(
                result.as_mut_ptr() as *mut _,
                document,
            );
            UniqueArc::assume_init(result).shareable()
        }
    }
}
// https://github.com/servo/servo/blob/f11c6045e33a921f03223c313781586189309bd2/components/style/properties/gecko.mako.rs#L496