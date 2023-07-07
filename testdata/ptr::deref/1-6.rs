unsafe fn load(ptr: &mut *const u128) -> Self {
    let ret = f32::from_bits(**ptr as u32);
    *ptr = (*ptr).add(1);
    return ret;
}
// https://github.com/paritytech/wasmtime/blob/8b5f28eff496b1085e1d86cb17bed7a8a9003c46/crates/api/src/func.rs#L882