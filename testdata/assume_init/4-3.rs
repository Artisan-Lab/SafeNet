/*
https://github.com/coolreader18/RustPython/blob/d03c7267d6ebe3eea8cc06b2ef1379c8e166acbe/vm/src/stdlib/time.rs#L264
*/
fn get_process_time(vm: &VirtualMachine) -> PyResult<Duration> {
    let t: libc::tms = unsafe {
        let mut t = std::mem::MaybeUninit::uninit();
        if libc::times(t.as_mut_ptr()) == -1 {
            return Err(vm.new_os_error("Failed to get clock time".to_owned()));         }
            t.assume_init()
    }; 
    let freq = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };
    Ok(Duration::from_nanos(
        time_muldiv(t.tms_utime, SEC_TO_NS, freq) + time_muldiv(t.tms_stime, SEC_TO_NS, freq),
    ))
}