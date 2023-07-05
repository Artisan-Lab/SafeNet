impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        loop {
            unsafe {
                let cur = self.cur.as_ref()?;
                self.cur = cur.ai_next;
                match sockaddr_to_addr(mem::transmute(cur.ai_addr), cur.ai_addrlen as usize) {
                    Ok(addr) => return Some(addr),
                    Err(_) => continue,
                }
            }
        }
    }
}
//https://github.com/Milo123459/rust/blob/0928a1f7574f5ca019b5443b3a90008588d18c8c/library/std/src/sys_common/net.rs#L155