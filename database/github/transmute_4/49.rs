pub extern "C" fn rs_snmp_state_new(_orig_state: *mut std::os::raw::c_void, _orig_proto: AppProto) -> *mut std::os::raw::c_void {
    let state = SNMPState::new();
    let boxed = Box::new(state);
    return unsafe{std::mem::transmute(boxed)};
}
/*
https://github.com/ms1111/suricata/blob/b3c1f2ab489c22494900476426fd5cad9ba08cd1/rust/src/snmp/snmp.rs#L300
*/