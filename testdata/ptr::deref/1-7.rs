unsafe fn store(abi: Self, ptr: *mut u128) {
    *ptr = abi.to_bits() as u128;
}
// https://github.com/paritytech/wasmtime/blob/8b5f28eff496b1085e1d86cb17bed7a8a9003c46/crates/api/src/func.rs#L882