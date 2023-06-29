pub extern "C" fn rs_sip_state_new() -> *mut std::os::raw::c_void {
    let state = SIPState::new();
    let boxed = Box::new(state);
    return unsafe { std::mem::transmute(boxed) };
}

/*
 https://github.com/mordak/suricata/blob/64a789bbf6b7e297048e574f21f7a084c54f307b/rust/src/sip/sip.rs#L178
*/