/*
    From:https://github.com/Clikengo/foundationdb-rs/blob/f41fa13eb57aa883944476e29ee81a6708e67b4d/foundationdb/src/future.rs#L128
*/
extern "C" fn fdb_future_callback( _f: *mut fdb_sys::FDBFuture, callback_parameter: *mut ::std::os::raw::c_void,) {
    let network_waker: Arc<AtomicWaker> = unsafe { Arc::from_raw(callback_parameter as *const _) };
    network_waker.wake();
}
