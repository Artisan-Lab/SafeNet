pub extern "C" fn rs_dhcp_logger_new(conf: *const c_void) -> *mut std::os::raw::c_void {
    let conf = ConfNode::wrap(conf);
    let boxed = Box::new(DHCPLogger::new(conf));
    return Box::into_raw(boxed) as *mut _;
}
