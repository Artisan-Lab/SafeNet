pub extern "C" fn rs_ntp_state_new(_orig_state: *mut std::os::raw::c_void, _orig_proto: AppProto) -> *mut std::os::raw::c_void {
    let state = NTPState::new();
    let boxed = Box::new(state);
    return unsafe{std::mem::transmute(boxed)};
}

/*
https://github.com/thus/suricata/blob/d00b755b647a69eb4d4a10adb57be45fd4d14c7d/rust/src/ntp/ntp.rs#L181
*/
