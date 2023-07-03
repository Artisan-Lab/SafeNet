pub extern "C" fn rs_ikev2_state_new() -> *mut libc::c_void {
    let state = IKEV2State::new();
    let boxed = Box::new(state);
    return unsafe{std::mem::transmute(boxed)};
}
/*
https://github.com/vipinpv85/DPDK_SURICATA-4_1_1/blob/974cc9eb54b0b1ab90eff12a95617e3e293b77d3/suricata-4.1.4/rust/src/ikev2/ikev2.rs#L442
*/