fn addr_v4() {
    let addr = "127.0.0.1:8080".parse().unwrap();

    let mut out: sockaddr_storage = unsafe { std::mem::zeroed() };

    assert_eq!(
        std_addr_to_c(&addr, &mut out),
        std::mem::size_of::<sockaddr_in>() as socklen_t
    );

    let s = std::ffi::CString::new("ddd.ddd.ddd.ddd").unwrap();

    let s = unsafe {
        let in_addr = &out as *const _ as *const sockaddr_in;
        assert_eq!(u16::from_be((*in_addr).sin_port), addr.port());

        let dst = s.into_raw();

        inet_ntop(
            AF_INET,
            &((*in_addr).sin_addr) as *const _ as *const c_void,
            dst,
            16,
        );

        std::ffi::CString::from_raw(dst).into_string().unwrap()
    };

    assert_eq!(s, "127.0.0.1");

    let addr = unsafe {
        std_addr_from_c(
            &*(&out as *const _ as *const sockaddr),
            std::mem::size_of::<sockaddr_in>() as socklen_t,
        )
    };

    assert_eq!(addr, "127.0.0.1:8080".parse().unwrap());
}


// https://github.com/cloudflare/quiche/blob/34a69e42b4b9974e8d21fc0165780da0331e7b14/quiche/src/ffi.rs#L1528