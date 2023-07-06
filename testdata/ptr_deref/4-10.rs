pub extern fn quiche_h3_conn_poll(
    conn: &mut h3::Connection, quic_conn: &mut Connection,
    ev: *mut *const h3::Event,
) -> i64 {
    match conn.poll(quic_conn) {
        Ok((id, v)) => {
            unsafe {
                *ev = Box::into_raw(Box::new(v));
            }

            id as i64
        },

        Err(e) => e.to_c() as i64,
    }
}

//https://github.com/bagder/quiche/blob/cf2a08757c942d13f15a5a22aa7ea9ef50309cbe/src/h3/ffi.rs#L97