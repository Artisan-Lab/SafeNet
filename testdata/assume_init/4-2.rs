/*
https://github.com/Pebaz/RustPython/blob/d1638f1a91087cb5128fd55da7e453a1a21f3a5e/vm/src/stdlib/resource.rs#L104
*/
   #[pyfunction]
    fn getrusage(who: i32, vm: &VirtualMachine) -> PyResult<Rusage> {
        let res = unsafe {
            let mut rusage = mem::MaybeUninit::<libc::rusage>::uninit();
            if libc::getrusage(who, rusage.as_mut_ptr()) == -1 {
                Err(io::Error::last_os_error())
            } else {
                Ok(rusage.assume_init())
            }
        };
        res.map(Rusage::from).map_err(|e| {
            if e.kind() == io::ErrorKind::InvalidInput {
                vm.new_value_error("invalid who parameter".to_owned())
            } else {
                e.into_pyexception(vm)
            }
        })
    }