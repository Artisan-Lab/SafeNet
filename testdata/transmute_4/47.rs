pub extern "C" fn rs_dhcp_logger_new(conf: *const c_void) -> *mut libc::c_void {
    let conf = ConfNode::wrap(conf);
    let boxed = Box::new(DHCPLogger::new(conf));
    return unsafe{std::mem::transmute(boxed)};
}
/*
https://github.com/tukapai/suricata/blob/ee576d4800576b7737d437c96fa06e476fee53b4/rust/src/dhcp/logger.rs#L259
*/
