pub extern "C" fn rs_template_state_new(_orig_state: *mut std::os::raw::c_void, _orig_proto: AppProto) -> *mut std::os::raw::c_void {
    let state = TemplateState::new();
    let boxed = Box::new(state);
    return unsafe { transmute(boxed) };
}
