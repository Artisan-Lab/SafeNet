pub extern "C" fn blake2b_free(state: *mut State) {
    if !state.is_null() {
        drop(unsafe { Box::from_raw(state) });
    }
}
// https://github.com/LayerTwo-Labs/zcash-sidechain/blob/93db677b91ced8accdf67ed796f06275d09c8af2/src/rust/src/blake2b.rs#L35