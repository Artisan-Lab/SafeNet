pub unsafe extern "C" fn sdp_free_session(sdp_ptr: *mut SdpSession) {
    let sdp = Rc::from_raw(sdp_ptr);
    drop(sdp);
}