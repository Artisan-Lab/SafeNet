fn unix_addr_from_unnamed_to_libc() {
    let addr = SockaddrUnix::new_unnamed();
    let addr = SockaddrStorage::from_unix(&addr.as_ref());
    let (ptr, len) = addr.as_ptr();
    let ptr = ptr as *const libc::sockaddr_un;
    let addr = unsafe { ptr.as_ref() }.unwrap();
}
/*
https://github.com/shadow/shadow/blob/a148ac8cee09f448959a413b05ca3d767641aa22/src/main/utility/sockaddr.rs#L768
*/