pub unsafe extern "C" fn pthread_rwlockattr_destroy(attr: *mut pthread_rwlockattr_t) -> c_int {
    core::ptr::drop_in_place(attr);

    0
}
#[no_mangle]
pub unsafe extern "C" fn pthread_rwlock_destroy(rwlock: *mut pthread_rwlock_t) -> c_int {
    core::ptr::drop_in_place(rwlock);

    0
}// https://github.com/redox-os/relibc/blob/1ef79540776f2290924e5d996ef38c13e9f1d505/src/header/pthread/rwlock.rs#L106 
