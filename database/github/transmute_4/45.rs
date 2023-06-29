pub extern "C" fn rs_tftp_state_alloc() -> *mut std::os::raw::c_void {
    let state = TFTPState { transactions : Vec::new(), tx_id: 0, };
    let boxed = Box::new(state);
    return unsafe{transmute(boxed)};
}
/*
https://github.com/delbs27/suricata/blob/26123e05f217596c58ec7a663455e34ba1eda582/rust/src/tftp/tftp.rs#L79
*/