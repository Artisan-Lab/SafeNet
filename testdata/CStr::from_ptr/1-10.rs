pub unsafe extern "C" fn selfprofile_before_pass_callback(
    llvm_self_profiler: *mut c_void,
    pass_name: *const c_char,
    ir_name: *const c_char,
) {
    let llvm_self_profiler = &mut *(llvm_self_profiler as *mut LlvmSelfProfiler<'_>);
    let pass_name = CStr::from_ptr(pass_name).to_str().expect("valid UTF-8");
    let ir_name = CStr::from_ptr(ir_name).to_str().expect("valid UTF-8");
    llvm_self_profiler.before_pass_callback(pass_name, ir_name);
}