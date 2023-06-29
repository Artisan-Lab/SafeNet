fn main() {
    quote! { #i => {
    match unsafe { *(*(a.offset(1)) as *const u32) } {
        #(#args)*
        _ => {}
        }
    }
    }
}