pub extern "C" fn rs_http2_state_new() -> *mut std::os::raw::c_void {
    let state = HTTP2State::new();
    let boxed = Box::new(state);
    return unsafe { transmute(boxed) };
}
/*
https://github.com/Zeraka/suricata/blob/d3cf2c21df625cfe9d3dcd605f110e3fb76e5601/rust/src/http2/http2.rs#L877
*/