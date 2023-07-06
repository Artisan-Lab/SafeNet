fn clone(&self) -> Self {
    use self::PropertyDeclaration::*;

    <%
        [copy, others] = [list(g) for _, g in groupby(variants, key=lambda x: not x["copy"])]
    %>

    let self_tag = unsafe {
        (*(self as *const _ as *const PropertyDeclarationVariantRepr<()>)).tag
    };
    if self_tag <= LonghandId::${copy[-1]["name"]} as u16 {
        #[derive(Clone, Copy)]
        #[repr(u16)]
        enum CopyVariants {
            % for v in copy:
            _${v["name"]}(${v["type"]}),
            % endfor
        }

        unsafe {
            let mut out = mem::MaybeUninit::uninit();
            ptr::write(
                out.as_mut_ptr() as *mut CopyVariants,
                *(self as *const _ as *const CopyVariants),
            );
            return out.assume_init();
        }
    }
}