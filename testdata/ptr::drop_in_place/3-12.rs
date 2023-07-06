pub unsafe extern "C" fn ipcData_destroy(ipc_data: *mut IPCData) {
    unsafe { std::ptr::drop_in_place(notnull_mut(ipc_data)) }
}
// https://github.com/shadow/shadow/blob/9838dfa31c1e3e878596ec45d615c77677d56c28/src/lib/shadow-shim-helper-rs/src/ipc.rs#L67