pub fn set_socket_opts(stream: &mut TcpStream) {
    if let Err(e) = stream.set_nodelay(true) {
        error!("set nodelay failed: {:?}", e);
    }

    // safety: we move the value out of stream and replace it at the end
    let ret = unsafe {
        let s = ptr::read(stream);
        let socket = Socket::from_raw_fd(s.into_raw_fd());
        let ret = socket.set_keepalive(true);
        ptr::write(stream, TcpStream::from_raw_fd(socket.into_raw_fd()));

        ret
    };

    if let Err(e) = ret {
        error!("set keepalive failed: {:?}", e);
    }
}


// https://github.com/fastly/pushpin/blob/9cb6b1d04184faf5315eadb92ad8a8a243987638/src/net.rs#L31