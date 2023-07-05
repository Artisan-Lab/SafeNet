unsafe extern "C" fn #local_drop #impl_generics(this: *mut ::cxx::alloc::boxed::Box<#ident #ty_generics>) {
    let __fn = concat!("<", module_path!(), #prevent_unwind_drop_label);
    ::cxx::private::prevent_unwind(__fn, || ::cxx::core::ptr::drop_in_place(this));
}
// https://github.com/dtolnay/cxx/blob/8308e991189ce6d8c2eb535a5369772d97ff8fb5/macro/src/expand.rs#L1294