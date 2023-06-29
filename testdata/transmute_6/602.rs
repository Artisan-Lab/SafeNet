fn test_unsafe() {
    unsafe {
        let cur = self.cur.as_ref()?;
        self.cur = cur.ai_next;
        match sockaddr_to_addr(mem::transmute(cur.ai_addr), cur.ai_addrlen as usize) {
            Ok(addr) => return Some(addr),
            Err(_) => continue,
        }
    }
}
