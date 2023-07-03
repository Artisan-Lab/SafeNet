pub extern "C" fn rs_ntp_state_new() -> *mut libc::c_void {
    let state = NTPState::new();
    let boxed = Box::new(state);
    return unsafe{std::mem::transmute(boxed)};
}
/*
https://github.com/JustinAzoff/suricata/blob/be3c7d5706a04c18685b123a3d61c51e010f5f87/rust/src/ntp/ntp.rs#L180
*/