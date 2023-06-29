fn main() {
    quote! { /* externally defined variables: arg_type, arg_index. */
    #fn_i => {
        match arg_index {
            #(#args)*
            _ => -1, // default when type is unknown
        }
    }
}
}