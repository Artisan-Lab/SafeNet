use std::ptr::NonNull;
fn getpwall(vm: &VirtualMachine) -> PyResult<Vec<PyObjectRef>> {
    while let Some(ptr) = NonNull::new(unsafe { libc::getpwent() }) {
        let user = User::from(unsafe { ptr.as_ref() });;
    }
}
/*
https://github.com/youknowone/RustPython/blob/5b1e92f7938371513849afcb6b91be653bf8e3f7/vm/src/stdlib/pwd.rs#L13
*/