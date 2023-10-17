pub fn recv_mmsg(sock: &UdpSocket, packets: &mut [Packet]) -> io::Result</*num packets:*/ usize> {
    // Assert that there are no leftovers in packets.
    debug_assert!(packets.iter().all(|pkt| pkt.meta() == &Meta::default()));
    const SOCKADDR_STORAGE_SIZE: usize = mem::size_of::<sockaddr_storage>();

    let mut hdrs: [mmsghdr; NUM_RCVMMSGS] = unsafe { mem::zeroed() };
    let iovs = mem::MaybeUninit::<[iovec; NUM_RCVMMSGS]>::uninit();
    let mut iovs: [iovec; NUM_RCVMMSGS] = unsafe { iovs.assume_init() };
    let mut addrs: [sockaddr_storage; NUM_RCVMMSGS] = unsafe { mem::zeroed() };

    let sock_fd = sock.as_raw_fd();
    let count = cmp::min(iovs.len(), packets.len());

    for (packet, hdr, iov, addr) in
        izip!(packets.iter_mut(), &mut hdrs, &mut iovs, &mut addrs).take(count)
    {
        let buffer = packet.buffer_mut();
        *iov = iovec {
            iov_base: buffer.as_mut_ptr() as *mut libc::c_void,
            iov_len: buffer.len(),
        };
        hdr.msg_hdr.msg_name = addr as *mut _ as *mut _;
        hdr.msg_hdr.msg_namelen = SOCKADDR_STORAGE_SIZE as socklen_t;
        hdr.msg_hdr.msg_iov = iov;
        hdr.msg_hdr.msg_iovlen = 1;
    }
    let mut ts = libc::timespec {
        tv_sec: 1,
        tv_nsec: 0,
    };
    #[allow(clippy::useless_conversion)]
    let nrecv = unsafe {
        libc::recvmmsg(
            sock_fd,
            &mut hdrs[0],
            count as u32,
            MSG_WAITFORONE.try_into().unwrap(),
            &mut ts,
        )
    };
    let nrecv = if nrecv < 0 {
        return Err(io::Error::last_os_error());
    } else {
        usize::try_from(nrecv).unwrap()
    };
    for (addr, hdr, pkt) in izip!(addrs, hdrs, packets.iter_mut()).take(nrecv) {
        pkt.meta_mut().size = hdr.msg_len as usize;
        if let Some(addr) = cast_socket_addr(&addr, &hdr) {
            pkt.meta_mut().set_socket_addr(&addr.to_std());
        }
    }
    Ok(nrecv)
}

// https://github.com/pyth-network/pythnet/blob/b12cecb197240715d519b82963d30bf095e2f9c2/streamer/src/recvmmsg.rs#L79