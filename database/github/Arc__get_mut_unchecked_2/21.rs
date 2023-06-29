fn accept_once(
    &mut self,
    poll: &Poll,
    pool: &mut IdlePool,
    udp_cache: &mut UdpSvrCache,
    resolver: &DnsResolver,
    net_profiler: &mut NetProfiler,
) -> Result<()> {
    let (size, src_addr, dst_addr) =
        sys::recv_from_with_destination(&self.udp_listener, self.recv_buffer.as_mut_slice())?;
    net_profiler.check(dst_addr.ip());
    log::debug!(
        "udp received {} byte from {} to {}",
        size,
        src_addr,
        dst_addr
    );
    let mut conn = if let Some(conn) = self.src_map.get(&src_addr) {
        log::debug!(
            "connection:{} already exists for address{}",
            conn.index,
            src_addr
        );
        conn.clone()
    } else {
        log::debug!(
            "address:{} not found, connecting to {}",
            src_addr,
            OPTIONS.back_addr.as_ref().unwrap()
        );
        if let Some(mut conn) = pool.get(poll, resolver) {
            if let Some(socket) = udp_cache.get_socket(dst_addr) {
                let index = next_index(&mut self.next_id);
                if !conn.reset_index(index, Token(index * CHANNEL_CNT + CHANNEL_UDP), poll) {
                    conn.check_status(poll);
                    return Ok(());
                }
                let mut conn = Connection::new(index, conn, src_addr, socket);
                if conn.setup() {
                    let conn = Arc::new(conn);
                    let _ = self.conns.insert(index, conn.clone());
                    self.src_map.insert(src_addr, conn.clone());
                    log::debug!("connection:{} is ready", index);
                    conn
                } else {
                    conn.check_status(poll);
                    return Ok(());
                }
            } else {
                conn.shutdown();
                conn.check_status(poll);
                return Ok(());
            }
        } else {
            log::error!("allocate connection failed");
            return Ok(());
        }
    };
    let payload = &self.recv_buffer.as_slice()[..size];
    unsafe { Arc::get_mut_unchecked(&mut conn) }.send_request(payload, &dst_addr, poll);
    if conn.destroyed() {
        self.removed.as_mut().unwrap().push(conn.index);
    }
    Ok(())
}
/*
https://github.com/Qv2ray/trojan-rs/blob/7b2850836ad580c9c3e87428eae6335533f1a187/src/proxy/udp_server.rs#L135
*/