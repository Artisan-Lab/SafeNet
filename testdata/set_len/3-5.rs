/*
https://github.com/dtolnay/watt/blob/f00b00dc8979f9b61202b529e99cbacc8f5816e3/jit/src/valtype.rs#L45
*/
pub fn new(mut list: Vec<ValType>) -> ValTypeVec {
        unsafe {
            let mut raw = mem::zeroed();
            ffi::wasm_valtype_vec_new(
                &mut raw,
                list.len(),
                list.as_ptr().cast::<*mut ffi::wasm_valtype_t>(),
            );
            list.set_len(0);
            ValTypeVec { raw }
        }
    }