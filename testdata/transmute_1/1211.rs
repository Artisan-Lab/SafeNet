pub extern "C" fn rs_mqtt_state_new(_orig_state: *mut std::os::raw::c_void, _orig_proto: AppProto) -> *mut std::os::raw::c_void {
    let state = MQTTState::new();
    let boxed = Box::new(state);
    return unsafe { transmute(boxed) };
}
/*
https://github.com/kramse/suricata/blob/b8499de498f0e31bb31097ac8b959e3877573de0/rust/src/mqtt/mqtt.rs#L584
*/