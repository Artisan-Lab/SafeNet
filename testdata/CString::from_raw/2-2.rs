unsafe fn build_config(mut self) -> *mut grpcio_sys::grpc_ssl_server_certificate_config {
    let root_cert = self
        .root
        .take()
        .map_or_else(ptr::null_mut, CString::into_raw);
    let cfg = grpcio_sys::grpc_ssl_server_certificate_config_create(
        root_cert,
        self.key_cert_pairs.as_ptr(),
        self.key_cert_pairs.len(),
    );
    if !root_cert.is_null() {
        drop(CString::from_raw(root_cert));
    }
    cfg
}

// https://github.com/tikv/grpc-rs/blob/4b76b8612c7bc5002aca00b14f33baf4f8235f0f/src/security/credentials.rs#L181